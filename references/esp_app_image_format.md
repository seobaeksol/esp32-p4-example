# ESP32-P4 ESP-IDF App Image Format Notes

이 문서는 현재 프로젝트가 `esp-hal` 없이 직접 만든 PAC와 `no_std` 애플리케이션을 유지하면서도, ESP-IDF 2nd stage bootloader/`espflash`가 인식할 수 있는 application image를 만들기 위해 필요한 최소 사항을 정리한다.

## 1. 배경

ESP32-P4의 SPI Boot mode에서 일반적인 boot 흐름은 다음과 같다.

```text
ROM bootloader → 2nd stage bootloader → application image
```

`espflash flash` 또는 `espflash save-image`는 ELF를 ESP-IDF 호환 application image로 변환한다. 이때 기본적으로 ELF 안에 ESP-IDF application descriptor가 있는지 검사한다.

현재 프로젝트는 `esp-hal`을 사용하지 않고 다음 구성으로 동작한다.

```text
custom PAC + esp-riscv-rt + no_std + 직접 작성한 low-level 코드
```

따라서 ESP-IDF bootloader 호환에 필요한 metadata만 직접 제공한다.

## 2. ESP-IDF application descriptor

ESP-IDF application image에는 `esp_app_desc`라는 metadata block이 포함된다. 이 block은 ELF의 `.flash.appdesc` section에 배치되어야 하며, bootloader/tooling이 앱 정보를 읽는 데 사용한다.

현재 프로젝트는 `src/esp_app_desc.rs`에서 이 구조체를 직접 정의한다.

핵심 조건:

| 항목 | 값 |
|---|---|
| exported symbol | `esp_app_desc` |
| link section | `.flash.appdesc` |
| size | 256 bytes |
| magic word | `0xABCD5432` |

현재 ELF에서 확인된 배치 예:

```text
.flash.appdesc  0x40005600  size 0x100
esp_app_desc    0x40005600  size 256
```

## 3. 왜 `esp-bootloader-esp-idf`를 쓰지 않는가?

`esp-bootloader-esp-idf` crate는 `esp_app_desc!()` macro를 제공하지만, ESP32-P4 지원이 들어간 main branch 기준으로 `esp-hal` dependency가 함께 들어온다.

이 프로젝트의 목표는 다음을 유지하는 것이다.

- 직접 생성한 PAC 사용
- `esp-hal` 초기화/드라이버 미사용
- `no_std` bare-metal boot/runtime 이해
- 필요한 boot metadata만 최소 구현

따라서 `esp_app_desc`만 직접 구현한다.

## 4. 현재 구현 요약

`src/main.rs`:

```rust
mod esp_app_desc;
```

`src/esp_app_desc.rs`:

```rust
#[unsafe(export_name = "esp_app_desc")]
#[unsafe(link_section = ".flash.appdesc")]
#[used]
pub static ESP_APP_DESC: EspAppDesc = EspAppDesc { ... };
```

descriptor의 주요 field:

| Field | 현재 값 |
|---|---|
| `magic_word` | `0xABCD5432` |
| `secure_version` | `0` |
| `version` | `CARGO_PKG_VERSION` |
| `project_name` | `CARGO_PKG_NAME` |
| `idf_ver` | `bare-metal` |
| `mmu_page_size` | `16`, 즉 `log2(64 KiB)` |
| `app_elf_sha256` | zero-filled |

`app_elf_sha256`는 현재 tooling에서 직접 채우지 않아도 image 생성은 통과한다.

## 5. 검증 명령

빌드:

```bash
cargo build
```

ESP-IDF application image 생성:

```bash
espflash save-image \
  --chip esp32p4 \
  --flash-size 16mb \
  --flash-mode qio \
  --flash-freq 80mhz \
  target/riscv32imafc-unknown-none-elf/debug/esp32-p4-example \
  /tmp/esp32-p4-example-appdesc.bin
```

성공 예:

```text
Chip type:         esp32p4
App/part. size:    89,680/16,384,000 bytes, 0.55%
Image successfully saved!
```

ELF section/symbol 확인:

```bash
readelf -S target/riscv32imafc-unknown-none-elf/debug/esp32-p4-example | rg 'appdesc|text|rodata'
readelf -s target/riscv32imafc-unknown-none-elf/debug/esp32-p4-example | rg 'esp_app_desc'
```

## 6. Flashing 관점

application image descriptor는 ESP-IDF bootloader 호환 image 생성을 위한 조건 중 하나일 뿐이다.

실제 ESP-IDF bootloader 흐름으로 flash하려면 추가로 필요하다.

- `partitions.csv`
- bootloader binary
  - `espflash` 기본 bootloader 사용, 또는
  - ESP-IDF로 직접 빌드한 `bootloader.bin`
- app partition offset/size 결정

예상 흐름:

```bash
espflash flash \
  --chip esp32p4 \
  --flash-size 16mb \
  --flash-mode qio \
  --flash-freq 80mhz \
  --partition-table partitions.csv \
  target/riscv32imafc-unknown-none-elf/release/esp32-p4-example
```

## 7. 주의사항

- 이 문서는 ESP-IDF image format 전체를 재구현하는 문서가 아니다.
- 현재 구현은 ESP-IDF app descriptor만 직접 제공한다.
- secure boot, flash encryption, OTA, anti-rollback은 아직 다루지 않는다.
- `mmu_page_size`는 descriptor metadata 값이며, 실제 ESP32-P4 bootloader/MMU 설정과 맞는지는 이후 보드 검증으로 확인해야 한다.
- `espflash save-image` 통과는 image 생성 조건을 만족한다는 의미이며, 실제 boot 성공은 partition table/bootloader/flash 설정까지 맞아야 한다.

## 8. 다음 작업

1. `partitions.csv` 추가
2. `.cargo/config.toml` runner에 ESP32-P4 flash 옵션 명시
3. bootloader binary를 `espflash` 기본값으로 둘지, ESP-IDF 빌드 산출물로 고정할지 결정
4. 실제 보드에서 SPI Boot mode 부팅 확인
