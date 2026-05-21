# ESP32-P4 Boot Log Analysis: eFuse Block Revision Rejection

분석 대상 로그 날짜: 2026-05-21

## 1. 요약

이번 로그에서는 ESP-IDF 2nd stage bootloader가 정상적으로 실행되고, partition table도 정상적으로 읽었다.

이전 linker 문제에서 보였던 `vaddr=00000000` dummy segment나 `rom_index == 2` assert는 나타나지 않는다. 즉 `memory.x`에서 flash origin을 `0x40000020`으로 바꾼 효과는 확인된다.

현재 부팅 실패의 직접 원인은 다음 메시지다.

```text
E (109) boot_comm: Image requires efuse blk rev >= v0.4, but chip is v0.3
E (117) boot: Factory app partition is not bootable
```

즉 factory app image가 현재 보드의 eFuse block revision보다 높은 revision을 요구한다고 판단되어 bootloader가 실행을 거부했다.

## 2. 로그 흐름

### 2.1 ROM bootloader 단계

```text
ESP-ROM:esp32p4-eco2-20240710
Build:Jul 10 2024
rst:0x3 (SW_SYS_RESET),boot:0xc (SPI_FAST_FLASH_BOOT)
SPI mode:QIO, clock div:1
```

보드는 SPI flash boot mode로 진입했다. reset 원인은 software system reset이다.

```text
Core0 Saved PC:0x4ff2e7fe
0x4ff2e7fe - g_shared_buffers
```

이전 실행 중 내부 RAM 영역의 주소가 saved PC로 남아 있다. 이번 실패의 핵심 원인은 아니고 reset 직전 상태를 보여주는 정보로 보면 된다.

### 2.2 2nd stage bootloader 로드

```text
load:0x4ff33ce0,len:0x15e4
load:0x4ff29ed0,len:0xe28
load:0x4ff2cbd0,len:0x3560
entry 0x4ff29eda
I (24) boot: ESP-IDF v5.5.3 2nd stage bootloader
```

ROM bootloader가 ESP-IDF 2nd stage bootloader를 내부 RAM으로 로드했고, bootloader 진입에 성공했다.

### 2.3 칩/flash 정보

```text
I (26) boot: chip revision: v1.3
I (27) boot: efuse block revision: v0.3
I (31) boot.esp32p4: SPI Speed      : 80MHz
I (35) boot.esp32p4: SPI Mode       : QIO
I (39) boot.esp32p4: SPI Flash Size : 16MB
```

중요한 값은 다음 두 개다.

| 항목 | 값 |
|---|---:|
| chip revision | `v1.3` |
| eFuse block revision | `v0.3` |

이번 실패 메시지는 chip revision이 아니라 **eFuse block revision** 기준이다.

### 2.4 partition table 확인

```text
I (47) boot: Partition Table:
I (56) boot:  0 nvs              WiFi data        01 02 00009000 00004000
I (62) boot:  1 otadata          OTA data         01 00 0000d000 00002000
I (69) boot:  2 phy_init         RF data          01 01 0000f000 00001000
I (75) boot:  3 factory          factory app      00 00 00010000 00100000
I (82) boot:  4 ota_0            OTA app          00 10 00110000 00100000
I (88) boot:  5 ota_1            OTA app          00 11 00210000 00100000
I (96) boot: End of partition table
```

`partitions.csv`와 일치한다.

```text
0x9000    nvs
0xd000    otadata
0xf000    phy_init
0x10000   factory
0x110000  ota_0
0x210000  ota_1
```

따라서 partition table offset/layout 문제는 아니다.

### 2.5 factory image 선택

```text
I (99) boot: Defaulting to factory image
```

`otadata`에서 유효한 OTA 선택 정보가 없어서 factory app을 선택했다. 현재 단계에서는 정상적인 흐름이다.

### 2.6 app image segment 확인

```text
I (102) esp_image: segment 0: paddr=00010020 vaddr=40000020 size=00c8ch (  3212) map
```

중요한 점:

```text
paddr = 0x00010020
vaddr = 0x40000020
```

factory partition offset은 `0x10000`이고, ESP image header 크기 `0x20` 뒤부터 첫 segment가 시작한다.

```text
0x10000 + 0x20 = 0x10020
```

`vaddr`도 `0x40000020`이므로 paddr/vaddr low bits가 맞는다. 이전 분석에서 문제였던 `vaddr=00000000` padding segment는 사라졌다.

이는 `memory.x` 변경이 의도대로 적용됐다는 긍정적인 신호다.

## 3. 실패 원인

직접 원인:

```text
Image requires efuse blk rev >= v0.4, but chip is v0.3
```

bootloader가 app image metadata를 검사하는 과정에서, image가 최소 eFuse block revision `v0.4` 이상을 요구한다고 판단했다. 하지만 실제 칩은 `v0.3`이므로 factory image를 bootable하지 않다고 처리했다.

이후 OTA partition도 확인하지만, 둘 다 비어 있으므로 최종적으로 부팅 가능한 app이 없다고 종료한다.

```text
E (120) esp_image: image at 0x110000 has invalid magic byte (nothing flashed here?)
E (128) boot: OTA app partition slot 0 is not bootable
E (133) esp_image: image at 0x210000 has invalid magic byte (nothing flashed here?)
E (140) boot: OTA app partition slot 1 is not bootable
E (145) boot: No bootable app partitions in the partition table
```

OTA 쪽 메시지는 부가 결과다. 핵심 실패는 factory image의 eFuse block revision check다.

## 4. 현재 코드베이스와의 연결

현재 app descriptor는 `src/esp_app_desc.rs`에서 직접 정의한다.

```rust
min_efuse_blk_rev_full: 0,
max_efuse_blk_rev_full: u16::MAX,
```

따라서 Rust source 상으로는 app descriptor가 `v0.4`를 요구하지 않는다.

현재 release image를 `espflash save-image`로 생성해 확인하면 app descriptor 내부 값도 다음과 같다.

```text
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 65535
mmu_page_size = 16
```

즉 이 로그만 기준으로 보면 가능한 원인은 다음 중 하나다.

1. 실제 flash된 image가 현재 source/build 산출물과 다르다.
2. `espflash`가 ESP image header 또는 다른 metadata에 revision requirement를 넣고 있다.
3. ESP-IDF v5.5.3 bootloader가 ESP32-P4용 image header/app metadata를 해석하는 방식과 현재 수동 image descriptor 구성이 완전히 일치하지 않는다.
4. 사용 중인 기본 bootloader와 `espflash`가 생성한 app image의 ESP32-P4 revision metadata 조합에 mismatch가 있다.

## 5. 현재까지 해결된 것과 남은 것

해결된 것으로 보이는 항목:

- ESP-IDF 2nd stage bootloader 진입 성공
- partition table 인식 성공
- factory partition 선택 성공
- first mapped segment의 `paddr`/`vaddr` 정렬 문제 해결
- 이전 `rom_index == 2` assert 재현 안 됨

남은 blocker:

- app image가 eFuse block revision `v0.4` 이상을 요구한다고 판단되는 이유 확인
- 실제 on-flash image와 현재 build image가 같은지 확인
- ESP-IDF v5.5.3 bootloader/espflash/app descriptor metadata 호환성 확인

## 6. 다음 확인 명령

### 6.1 release image 재생성

```bash
cargo build --release

espflash save-image \
  --chip esp32p4 \
  --flash-size 16mb \
  --flash-mode qio \
  --flash-freq 80mhz \
  target/riscv32imafc-unknown-none-elf/release/esp32-p4-example \
  /tmp/esp32-p4-example-release.bin
```

### 6.2 app descriptor 값 확인

현재 확인된 release image 기준:

```text
segment 0 addr = 0x40000020
segment 0 size = 0x0c8c
appdesc offset = 0x0a68
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 65535
mmu_page_size = 16
```

### 6.3 flash 시 명시 옵션 유지

```bash
espflash flash \
  --chip esp32p4 \
  --flash-size 16mb \
  --flash-mode qio \
  --flash-freq 80mhz \
  --partition-table partitions.csv \
  target/riscv32imafc-unknown-none-elf/release/esp32-p4-example
```

필요하면 `--min-chip-rev 0.0`도 명시해서 image header 쪽 revision metadata가 바뀌는지 확인한다.

```bash
espflash flash \
  --chip esp32p4 \
  --flash-size 16mb \
  --flash-mode qio \
  --flash-freq 80mhz \
  --min-chip-rev 0.0 \
  --partition-table partitions.csv \
  target/riscv32imafc-unknown-none-elf/release/esp32-p4-example
```

## 7. 결론

이번 로그는 linker layout 수정이 진전됐음을 보여준다. bootloader는 app partition의 첫 segment를 다음과 같이 정상적으로 map하려고 한다.

```text
paddr=0x10020, vaddr=0x40000020
```

현재 실패는 memory layout 문제가 아니라 image metadata/revision compatibility 문제다. 다음 작업은 flash된 app image의 ESP image header와 app descriptor를 분리해서 확인하고, bootloader가 `efuse blk rev >= v0.4` 요구를 어디서 읽는지 좁히는 것이다.
