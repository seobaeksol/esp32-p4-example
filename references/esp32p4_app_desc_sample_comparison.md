# ESP32-P4 App Description Comparison: Manufacturer Sample vs Current Image

비교 대상:

- 제조사 샘플: `references/jc4880p443_samples/P4-JC4880P443-V1.8.bin`
- 현재 프로젝트 release image: `/tmp/esp32-p4-example-release.bin` 생성 기준

## 1. 결론

제조사 샘플과 현재 image를 비교한 결과, bootloader 에러의 원인을 좁힐 수 있었다.

현재 boot 로그의 핵심 에러:

```text
E boot_comm: Image requires efuse blk rev >= v0.4, but chip is v0.3
```

이는 실제 `src/esp_app_desc.rs`의 `min_efuse_blk_rev_full = 0` 값 때문이 아니라, **ESP-IDF bootloader가 app descriptor를 기대하는 위치에서 현재 image의 machine code를 app descriptor처럼 읽고 있기 때문**으로 보인다.

제조사 샘플은 app descriptor가 app image payload의 맨 앞, 즉 app offset `+ 0x20`에 있다.

현재 프로젝트 image는 app descriptor가 app image 내부 `0xa68`에 있다. 따라서 bootloader가 fixed/expected 위치인 `0x20`에서 descriptor를 읽으면, 실제 descriptor가 아니라 `.text` code bytes를 읽게 된다.

그 code bytes를 `esp_app_desc_t`로 해석하면 `min_efuse_blk_rev_full = 4`가 나오며, 이것이 로그의 `requires efuse blk rev >= v0.4`와 일치한다.

## 2. 제조사 샘플 flash layout

샘플 binary 크기:

```text
0xd20000 bytes
```

partition table은 `0x8000`에 있다.

```text
nvs       data  nvs      offset=0x0000b000 size=0x00006000
phy_init  data  phy      offset=0x00011000 size=0x00001000
factory   app   factory  offset=0x00020000 size=0x00700000
storage   data  0x82     offset=0x00720000 size=0x00600000
```

현재 우리 partition table과는 다르다.

| 항목 | 제조사 샘플 | 현재 프로젝트 |
|---|---:|---:|
| partition table offset | `0x8000` | `0x8000` |
| factory app offset | `0x20000` | `0x10000` |
| factory app size | `0x700000` | `0x100000` |

## 3. 제조사 샘플 app image header

제조사 샘플의 factory app은 `0x20000`에서 시작한다.

```text
IMG at 0x20000
magic   = 0xe9
segments = 7
mode    = 0x2
config  = 0x4f
entry   = 0x4ff0040e
chip_id = 0x12
min_chip_rev_full = 1
max_chip_rev_full = 199
```

segment 배치는 다음과 같다.

```text
segment 0 addr=0x481d0020 size=0x415aa4 data_off=0x20020
segment 1 addr=0x30100000 size=0x68     data_off=0x435acc
segment 2 addr=0x4ff00000 size=0xa4dc   data_off=0x435b3c
segment 3 addr=0x48000020 size=0x1ccd60 data_off=0x440020
segment 4 addr=0x4ff0a4dc size=0x123ec  data_off=0x60cd88
segment 5 addr=0x4ff1c900 size=0x4230   data_off=0x61f17c
segment 6 addr=0x50108080 size=0x20     data_off=0x6233b4
```

중요한 점은 segment 0의 data가 `0x20020`에서 시작하고, 여기에 app descriptor가 바로 위치한다는 점이다.

```text
app image offset = 0x20000
app descriptor   = 0x20020
relative offset  = 0x20
```

## 4. 제조사 샘플 app descriptor

샘플의 `esp_app_desc` magic `0xABCD5432`는 `0x20020`에서 발견된다.

```text
magic      = 0xABCD5432
version    = 1.0.1
project    = esp_brookesia_demo
time       = 09:31:36
date       = Jan 15 2026
idf_ver    = v5.5.1-dirty
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 99
mmu_page_size = 16
```

즉 제조사 샘플은 eFuse block revision 최소 요구값이 `0`이다.

## 5. 현재 프로젝트 image의 app descriptor 위치

현재 프로젝트 release image 생성 결과:

```text
segments = 1
entry    = 0x40000022
segment 0 addr=0x40000020 size=0x0c8c data_off=0x20
```

실제 `esp_app_desc` magic은 image offset `0xa68`에서 발견된다.

```text
app descriptor offset = 0x0a68
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 65535
mmu_page_size = 16
```

따라서 source에 정의한 descriptor 값 자체는 다음처럼 정상이다.

```rust
min_efuse_blk_rev_full: 0,
max_efuse_blk_rev_full: u16::MAX,
mmu_page_size: 16,
```

하지만 descriptor 위치가 ESP-IDF bootloader가 기대하는 위치와 다르다.

## 6. 왜 `efuse blk rev >= v0.4`가 나오는가

현재 프로젝트 image의 app payload 시작은 `0x20`이다.

이 위치는 실제로 `.text` code 시작이다. 이 위치를 잘못 `esp_app_desc_t`로 해석하면 다음 값이 나온다.

```text
pos 0x20 magic = 0x00b78082   # app desc magic 아님
fields at +176:
min_efuse_blk_rev_full = 4
max_efuse_blk_rev_full = 34195
mmu_page_size = 4
```

`min_efuse_blk_rev_full = 4`는 ESP-IDF 표시 형식으로 `v0.4`이다.

따라서 bootloader 로그의 다음 메시지와 정확히 맞는다.

```text
Image requires efuse blk rev >= v0.4, but chip is v0.3
```

즉 bootloader가 실제 descriptor `0xa68`이 아니라 payload 시작 `0x20` 부근의 code bytes를 descriptor로 오해하고 있다.

## 7. 현재 linker 상태의 문제

현재 `esp-idf-sections.x`는 `.flash.appdesc`를 `.rodata` 앞에 두지만, `riscv-rt`의 `.text` section은 여전히 먼저 배치된다.

현재 ELF section 순서:

```text
.text             0x40000020
.flash.appdesc    0x40000a68   # release 기준 image offset 0xa68
.rodata           ...
```

ESP-IDF/esp-hal의 의도는 app descriptor가 flash image 초반에 오도록 하는 것이다. 제조사 샘플은 이를 더 강하게 만족하며, descriptor가 segment 0 payload 첫 위치에 있다.

## 8. 다음 수정 방향

다음 목표는 `.flash.appdesc`를 app image payload의 첫 위치로 이동하는 것이다.

목표 image 형태:

```text
image header      offset 0x00 ~ 0x17
segment header    offset 0x18 ~ 0x1f
esp_app_desc      offset 0x20
.text             그 이후
.rodata           그 이후
```

현재 `riscv-rt`의 기본 `link.x`가 `.text _stext`를 먼저 만들기 때문에, 단순히 `INSERT BEFORE .rodata`만으로는 부족하다.

가능한 접근:

1. custom linker script로 `.flash.appdesc`를 `.text`보다 먼저 명시한다.
2. `riscv-rt` 기본 `link.x` 의존을 줄이고 ESP-IDF bootloader용 `link-esp-idf.x`를 만든다.
3. 최소 실험으로 `.flash.appdesc` section을 `INSERT BEFORE .text`로 이동하되, `_stext`/entry/segment layout이 깨지지 않는지 확인한다.
4. app descriptor 전용 첫 segment를 만들고, 그 뒤에 `.text` segment를 배치하는 구조를 만든다.

## 9. 결론

제조사 샘플과 비교한 결과, 문제는 eFuse 값 자체가 아니라 **app descriptor 위치**다.

제조사 샘플:

```text
esp_app_desc at app + 0x20
```

현재 프로젝트:

```text
esp_app_desc at app + 0xa68
```

bootloader는 app 시작 직후 descriptor를 기대하고 있으며, 현재 image에서는 그 위치가 code bytes라서 `min_efuse_blk_rev_full = 4`로 오해한다. 이 때문에 `efuse blk rev >= v0.4` 에러가 발생한다.

## 10. 적용 결과: linker script 조정

다음 변경을 적용했다.

- `memory.x`
  - `_stext = ORIGIN(FLASH) + 0x100` 추가
  - app descriptor 256 bytes를 flash payload 맨 앞에 두고, 실제 `.text`는 그 뒤에서 시작
- `esp-idf-sections.x`
  - `.flash.appdesc`를 `INSERT BEFORE .text.dummy`로 배치
  - `riscv-rt`의 `.text.dummy`/`.text`보다 먼저 descriptor가 나오도록 조정

검증 결과 debug image:

```text
segments = 1
entry    = 0x40000122
segment0 addr=0x40000020 size=0x5e00 data_off=0x20
appdesc offset = 0x20
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 65535
mmu_page_size = 16
```

ELF section 순서도 다음처럼 바뀌었다.

```text
.flash.appdesc  0x40000020 size=0x100
.text.dummy     0x40000120 size=0
.text           0x40000120
.rodata         ...
```

즉 현재 image는 제조사 샘플과 동일하게 app descriptor가 app image payload 첫 위치에 온다.

```text
ESP image header   offset 0x00 ~ 0x17
segment header     offset 0x18 ~ 0x1f
esp_app_desc       offset 0x20
.text              offset 0x120부터
```

따라서 이전 bootloader 에러의 원인이었던 `offset 0x20의 code bytes를 app descriptor로 오해하는 문제`는 해결된 것으로 판단한다.

## 11. 추가 조정: ESP32-P4 IROM range 사용

이후 `rom_index == 2` assert를 보고 flash-mapped executable address range를 다시 확인했다. `espflash`의 ESP32-P4 address classifier는 다음처럼 0x4800_0000 range를 IROM으로 분류한다.

```text
0x4800_0000..0x4C00_0000  IROM
0x4000_0000..0x4400_0000  DROM
```

따라서 현재 프로젝트의 executable `.text`를 `0x40000020`에 두는 것은 DROM 쪽에 code를 둔 형태가 된다. 이를 피하기 위해 app image 전체를 IROM range로 옮겼다.

적용 후 release image:

```text
.flash.appdesc  0x48000020 size=0x100
.text           0x48000120
.rodata         0x48000b68
entry           0x48000122
```

생성된 ESP image:

```text
segments = 1
segment0 addr=0x48000020 size=0x0c8c data_off=0x20
appdesc offset = 0x20
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 65535
mmu_page_size = 16
```

이 변경은 app descriptor 위치를 유지하면서 bootloader가 executable image를 IROM segment로 보도록 하기 위한 실험이다.

## 12. 적용 결과: DROM + IROM 두 mapped segment 구성

ESP-IDF bootloader의 `unpack_load_app()`는 ESP32-P4에서 flash/PSRAM mapped segment를 정확히 2개 기대한다.

```c
assert(rom_index < 2);
...
assert(rom_index == 2);
```

이에 맞춰 linker layout을 다음처럼 조정했다.

```text
segment 0: DROM 0x40000020
  - esp_app_desc at image offset 0x20
  - padding to force the next segment payload offset

segment 1: IROM 0x4800002c
  - small gap up to 0x48000120
  - .text at 0x48000120
  - .rodata/.eh_frame after .text
```

`espflash`는 첫 flash segment가 MMU page boundary에 너무 가깝게 끝나면 0x0c bytes를 추가 padding한다. 따라서 linker상 DROM section은 `0xfff8` bytes로 만들고, espflash가 최종 image에서는 `0x10004` bytes로 저장하게 둔다.

그 결과 두 번째 segment의 payload file offset은 `0x1002c`가 된다. 이를 맞추기 위해 `IROM` origin을 `0x4800002c`로 두었다.

검증된 release image:

```text
segments = 2
entry    = 0x48000122

segment 0 addr=0x40000020 size=0x10004 data_off=0x20
segment 1 addr=0x4800002c size=0x0c80  data_off=0x1002c

appdesc offset = 0x20
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 65535
mmu_page_size = 16
```

ELF section 배치:

```text
.flash.appdesc  0x40000020 size=0xfff8
.irom_start_gap 0x4800002c size=0x00f4
.text           0x48000120
.rodata         0x48000b68
.eh_frame       0x48000c20
```

이제 bootloader의 `rom_index == 2` 조건에 맞는 image가 생성된다.
