# esp-hal ESP32-P4 Linker Script Analysis

Reference source:

- Repository: `https://github.com/esp-rs/esp-hal`
- Commit: `50f4e5844ee4f1a8576b59ddebe7cb119aa57aef`
- Original path: `esp-hal/ld/esp32p4`
- Local copy: `references/esp_hal/ld/esp32p4`

Copied files:

- `esp32p4.x`
- `linkall.x`
- `memory.x`

## 1. High-level difference

현재 프로젝트의 `memory.x`는 단순한 TRM address map 기반이다.

```ld
FLASH : ORIGIN = 0x40000000, LENGTH = 16M
RAM   : ORIGIN = 0x4FF00000, LENGTH = 768K
```

반면 `esp-hal`의 ESP32-P4 linker setup은 ESP-IDF 2nd stage bootloader가 이미 설정한 상태를 전제로 한다.

핵심 차이:

1. external flash region 시작이 `0x40000000`이 아니라 `0x40000020`
2. HP L2MEM 전체를 RAM으로 쓰지 않고, L2 cache 영역과 ROM BSS/stack 영역을 피함
3. `.flash.appdesc`가 flash image에서 먼저 나오도록 section ordering을 강제함
4. `.text`와 `.rodata`를 별도 flash segment로 만들기 위한 gap을 둠
5. `riscv-rt` alias 외에도 `esp-hal` 전용 alias를 사용함

## 2. `memory.x` 비교

### 2.1 Flash region

`esp-hal`:

```ld
ROM : ORIGIN = 0x40000000 + 0x20, LENGTH = 0x400000 - 0x20
```

현재 프로젝트:

```ld
FLASH : ORIGIN = 0x40000000, LENGTH = 16M
```

`esp-hal` 주석:

```text
External flash (XIP via cache); +0x20 skips the IDF app image header.
```

이 차이가 현재 boot failure와 직접 관련이 있을 가능성이 높다.

현재 ESP-IDF bootloader 로그:

```text
I esp_image: segment 0: paddr=00010020 vaddr=00000000 size=0xffd8
I esp_image: segment 1: paddr=00020000 vaddr=40000000 size=00cac map
Assert failed in unpack_load_app, bootloader_utility.c:843 (rom_index == 2)
```

해석:

- app partition offset은 `0x10000`
- ESP image header 이후 첫 segment payload는 `0x10020`에서 시작
- 그런데 현재 ELF의 flash-mapped code는 `vaddr=0x40000000`
- `paddr` low bits는 `0x20`, `vaddr` low bits는 `0x00`이라 MMU page alignment가 맞지 않음
- `espflash`가 이를 맞추기 위해 `vaddr=0x00000000` dummy/padding segment를 `0x10020`부터 `0x20000`까지 만든 것으로 보임
- ESP-IDF bootloader가 이 segment layout을 ESP32-P4 정상 image로 받아들이지 못하고 assert 발생

`esp-hal`처럼 ROM origin을 `0x40000020`으로 두면:

```text
paddr 0x10020 low bits == vaddr 0x40000020 low bits
```

이 되어 불필요한 `vaddr=0` padding segment를 피할 수 있다.

### 2.2 Flash size

`esp-hal`:

```ld
ROM LENGTH = 0x400000 - 0x20
```

즉 linker 관점에서 4 MiB만 ROM region으로 둔다.

현재 프로젝트:

```ld
FLASH LENGTH = 16M
```

보드 flash는 16 MB이지만, 현재 `partitions.csv`의 app partition은 1 MiB이다.

```csv
factory, app, factory, 0x10000, 0x100000
```

따라서 ESP-IDF bootloader 흐름에서는 linker의 ROM length도 실제 app partition size를 넘지 않게 관리하는 편이 안전하다.

현재 앱은 작아서 당장 overflow는 아니지만, linker가 16 MB 전체를 쓸 수 있다고 믿는 점은 partition table과 불일치한다.

### 2.3 RAM region

`esp-hal`:

```ld
RAM : ORIGIN = 0x4FF00000 + RESERVED_L2_CACHE,
      LENGTH = 0x4FFAE000 - RESERVED_L2_CACHE - 0x4FF00000
```

주석상 ESP32-P4 HP L2MEM 768 KiB는 다음 용도로 나뉜다.

- low end: L2 cache
- remaining area: L2 RAM
- top ~72 KiB: ROM BSS/stack 영역으로 보존

`esp-hal`은 L2 cache size에 따라 RAM origin을 바꾼다.

| L2 cache size | RAM origin |
|---:|---:|
| 512 KiB | `0x4FF80000` |
| 256 KiB | `0x4FF40000` |
| 128 KiB, IDF default | `0x4FF20000` |
| 0 KiB | `0x4FF00000` |

IDF default 128 KiB cache 기준 RAM 범위:

```text
0x4FF20000 ~ 0x4FFADFFF
size = 0x8E000, 약 568 KiB
```

현재 프로젝트:

```ld
RAM : ORIGIN = 0x4FF00000, LENGTH = 768K
```

이는 다음 영역을 침범할 수 있다.

- ESP-IDF bootloader가 설정한 L2 cache 영역
- ROM BSS/stack 보존 영역

특히 현재 ELF section은 `.stack`이 L2MEM 대부분을 차지한다.

```text
.bss    0x4ff00000
.stack  0x4ff00020 size 0xbffe0
```

ESP-IDF bootloader 흐름에서는 이 배치가 위험하다.

## 3. `linkall.x` 비교

`esp-hal` `linkall.x`:

```ld
INCLUDE "memory.x"

REGION_ALIAS("ROTEXT", ROM);
REGION_ALIAS("RODATA", ROM);
REGION_ALIAS("RWTEXT", RAM);
REGION_ALIAS("RWDATA", RAM);
REGION_ALIAS("RTC_FAST_RWTEXT", RTC_FAST);
REGION_ALIAS("RTC_FAST_RWDATA", RTC_FAST);

REGION_ALIAS("REGION_TEXT", ROM);
REGION_ALIAS("REGION_RODATA", ROM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

INCLUDE "esp32p4.x"
INCLUDE "hal-defaults.x"
```

현재 프로젝트는 `riscv-rt`용 alias만 정의한다.

```ld
REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
```

`esp-hal`은 code/data section을 더 세분화한다.

- `ROTEXT`: flash executable text
- `RODATA`: flash read-only data
- `RWTEXT`: RAM executable text/trap
- `RWDATA`: RAM data/bss
- `RTC_FAST_*`: LP SRAM/RTC fast memory

## 4. `esp32p4.x` 분석

### 4.1 hart/interrupt defaults

```ld
PROVIDE(_max_hart_id = 1);
PROVIDE(interrupt0 = DefaultHandler);
PROVIDE(_mp_hook = default_mp_hook);
```

현재 프로젝트도 `_max_hart_id = 1`은 정의하지만, interrupt/mp hook 처리는 `esp-riscv-rt` 기본과 직접 작성한 stub에 의존한다.

### 4.2 RAM-resident sections first

`esp32p4.x`:

```ld
SECTIONS {
  INCLUDE "rwtext.x"
  INCLUDE "rwdata.x"
}
```

`rwtext.x`에는 RISC-V trap section도 포함된다.

```ld
.trap > RWTEXT
.rwtext > RWTEXT
```

즉 `esp-hal`은 trap/rwtext를 RAM에 둔다.

현재 프로젝트는 upstream `riscv-rt`의 `link.x`에 많이 의존하므로 trap/text 배치가 `esp-hal`과 다르다.

### 4.3 `.text_gap`

`esp32p4.x`:

```ld
SECTIONS {
  .text_gap (NOLOAD): {
    . = . + 8;
    . = ALIGN(4) + 0x20;
  } > ROM
}
INSERT BEFORE .text;
```

주석:

```text
Bootloader really wants to have separate segments for ROTEXT and RODATA.
Thus, we need to force a gap here.
```

ESP-IDF bootloader는 flash text와 rodata segment가 분리된 형태를 기대한다. 이 gap은 ELF/program header가 bootloader 친화적인 segment layout이 되도록 유도하는 장치다.

현재 프로젝트에는 이 gap이 없다.

### 4.4 rodata/text/appdesc ordering

`esp32p4.x`는 다음 순서로 include한다.

```ld
INCLUDE "rodata.x"
INCLUDE "text.x"
INCLUDE "rtc_fast.x"
INCLUDE "stack.x"
INCLUDE "metadata.x"
INCLUDE "eh_frame.x"
```

`rodata.x`의 첫 section은 `.flash.appdesc`이다.

```ld
.flash.appdesc : ALIGN(4)
{
    KEEP(*(.flash.appdesc));
    KEEP(*(.flash.appdesc.*));
} > RODATA
```

주석:

```text
For ESP App Description, must be placed first in image
```

현재 프로젝트의 ELF section order는 다음과 같다.

```text
.text             0x40000000
.init.rust        0x40003f6c
.rodata           0x40003f94
.flash.appdesc    0x40005600
.eh_frame         0x40005700
```

즉 `.flash.appdesc`가 `.rodata` 뒤에 있다. `espflash save-image` 검증은 통과하지만, ESP-IDF/ESP32-P4 bootloader가 기대하는 가장 이상적인 layout은 아니다.

## 5. 현재 failure와 가장 관련 큰 차이

가장 중요한 차이는 다음 두 가지다.

### 5.1 ROM origin `+0x20`

현재:

```ld
FLASH ORIGIN = 0x40000000
```

`esp-hal`:

```ld
ROM ORIGIN = 0x40000020
```

현재 bootloader 로그의 `vaddr=0` dummy segment는 이 차이로 설명된다.

### 5.2 L2MEM RAM reservation

현재:

```ld
RAM = entire 0x4FF00000~0x4FFBFFFF
```

`esp-hal` IDF default 기준:

```ld
RAM = 0x4FF20000~0x4FFADFFF
```

ESP-IDF bootloader를 거친 app이라면 현재 RAM 설정은 cache/ROM reserved 영역과 충돌할 수 있다.

## 6. 우리 코드베이스에 적용할 방향

ESP-IDF bootloader flow와 Direct Boot flow를 분리하는 것이 좋다.

예상 파일 구조:

```text
memory.x                         # 현재 실험용 또는 direct-boot용
memory-esp-idf-bootloader.x      # ESP-IDF bootloader 호환용
```

ESP-IDF bootloader 호환용에서는 최소한 다음을 반영해야 한다.

```ld
ROM : ORIGIN = 0x40000020, LENGTH = <app partition size> - 0x20
RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8E000   /* IDF default 128 KiB L2 cache 기준 */
RTC_FAST : ORIGIN = 0x50108000, LENGTH = 32K
```

그리고 section ordering도 보강해야 한다.

- `.flash.appdesc`를 flash image 초반에 배치
- `.text`/`.rodata` segment가 ESP-IDF bootloader가 기대하는 식으로 분리되게 gap 추가
- trap/rwtext를 RAM에 둘지 검토

## 7. 단기 실험 제안

1. `memory.x`에서 flash origin만 임시로 바꿔본다.

```ld
FLASH : ORIGIN = 0x40000020, LENGTH = 0x100000 - 0x20
```

2. RAM도 IDF default에 맞춰 줄인다.

```ld
RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8E000
```

3. 다시 build/save-image 후 bootloader segment 로그를 확인한다.

목표는 다음과 같은 `vaddr=0` segment가 사라지는지 보는 것이다.

```text
segment 0: paddr=00010020 vaddr=00000000 size=0xffd8
```

이 segment가 사라지거나 `vaddr=0x40000020` 계열로 바뀌면 현재 assert 원인에 가까워진다.

## 8. 주의

`esp-hal` linker scripts는 그대로 GNU ld에 넣기 어렵다. `#IF ... #ENDIF` 형태의 preprocessing directive와 여러 공통 section script에 의존한다.

따라서 그대로 복사해서 쓰기보다는, 현재 프로젝트에 필요한 핵심만 추출해서 별도 linker script로 재구성하는 편이 안전하다.
