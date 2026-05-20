# ESP32-P4 Partition Table Guide

이 문서는 현재 프로젝트에서 ESP-IDF bootloader 호환 flash layout을 정의하기 위한 `partitions.csv` 작성 기준을 정리한다.

## 1. 목적

현재 프로젝트는 다음 구성을 목표로 한다.

```text
ROM bootloader → ESP-IDF 2nd stage bootloader → factory/OTA app partition
```

이를 위해 ESP-IDF 형식의 partition table을 `partitions.csv`에 둔다.

현재 앱은 NVS/OTA/PHY partition을 직접 사용하지 않지만, 향후 기능 확장을 고려해서 다음 partition을 미리 포함한다.

- `nvs`
- `otadata`
- `phy_init`
- `factory`
- `ota_0`
- `ota_1`

## 2. 현재 partition table

`partitions.csv`:

```csv
# ESP-IDF Partition Table
# Name,   Type, SubType, Offset,   Size,     Flags
nvs,      data, nvs,     0x9000,   0x4000,
otadata,  data, ota,     0xd000,   0x2000,
phy_init, data, phy,     0xf000,   0x1000,
factory,  app,  factory, 0x10000,  0x100000,
ota_0,    app,  ota_0,   0x110000, 0x100000,
ota_1,    app,  ota_1,   0x210000, 0x100000,
```

## 3. Layout

| Partition | Type | SubType | Offset | Size | Range |
|---|---|---|---:|---:|---:|
| `nvs` | data | nvs | `0x9000` | `0x4000` | `0x9000` ~ `0xCFFF` |
| `otadata` | data | ota | `0xD000` | `0x2000` | `0xD000` ~ `0xEFFF` |
| `phy_init` | data | phy | `0xF000` | `0x1000` | `0xF000` ~ `0xFFFF` |
| `factory` | app | factory | `0x10000` | `0x100000` | `0x10000` ~ `0x10FFFF` |
| `ota_0` | app | ota_0 | `0x110000` | `0x100000` | `0x110000` ~ `0x20FFFF` |
| `ota_1` | app | ota_1 | `0x210000` | `0x100000` | `0x210000` ~ `0x30FFFF` |

현재 정의는 16 MB flash 중 `0x310000`까지 사용한다.

## 4. Offset 선택 이유

ESP-IDF 계열의 일반적인 flash layout은 다음과 유사하다.

```text
0x0000 또는 0x1000  bootloader 영역
0x8000              partition table
0x9000              data partitions 시작
0x10000             factory app 시작
```

현재 layout은 `factory` app을 `0x10000`에 배치하고, 그 앞의 `0x9000` ~ `0xFFFF` 영역에 data partition을 둔다.

이 구성에서는 각 partition이 서로 겹치지 않는다.

```text
nvs      0x9000  + 0x4000 = 0xD000
otadata  0xD000 + 0x2000 = 0xF000
phy_init 0xF000 + 0x1000 = 0x10000
factory  0x10000부터 시작
```

## 5. 현재 단계에서의 영향

현재 bare-metal 앱은 아직 NVS, OTA, PHY 데이터를 사용하지 않는다. 따라서 해당 data partition들은 비어 있어도 앱 실행에는 영향을 주지 않는다.

단, `otadata`에 이전 테스트의 찌꺼기가 있으면 bootloader가 `factory`가 아니라 `ota_0`/`ota_1`을 선택하려 할 수 있다. 초기 테스트 전에는 flash erase를 권장한다.

```bash
espflash erase-flash --chip esp32p4
```

또는 `otadata`만 지울 수 있다.

```bash
espflash erase-region --chip esp32p4 0xd000 0x2000
```

## 6. 검증 명령

CSV를 ESP-IDF binary partition table로 변환한다.

```bash
espflash partition-table --to-binary partitions.csv -o /tmp/partitions.bin
```

binary를 다시 사람이 읽을 수 있는 table로 출력한다.

```bash
espflash partition-table /tmp/partitions.bin
```

현재 검증 결과 table:

```text
Name      Type  SubType  Offset    Size      Flags
nvs       data  nvs      0x9000    0x4000
otadata   data  ota      0xd000    0x2000
phy_init  data  phy      0xf000    0x1000
factory   app   factory  0x10000   0x100000
ota_0     app   ota_0    0x110000  0x100000
ota_1     app   ota_1    0x210000  0x100000
```

## 7. Flash 명령 예

```bash
espflash flash \
  --chip esp32p4 \
  --flash-size 16mb \
  --flash-mode qio \
  --flash-freq 80mhz \
  --partition-table partitions.csv \
  target/riscv32imafc-unknown-none-elf/release/esp32-p4-example
```

특정 app partition에 쓰려면 `--target-app-partition`을 사용할 수 있다.

```bash
espflash flash \
  --chip esp32p4 \
  --partition-table partitions.csv \
  --target-app-partition factory \
  target/riscv32imafc-unknown-none-elf/release/esp32-p4-example
```

## 8. 주의사항

- 각 app partition size는 현재 `1 MiB`이다. 현재 앱 크기에는 충분하지만, LCD/font/image asset이 커지면 늘려야 한다.
- OTA를 실제로 사용하려면 앱에서 `otadata`를 갱신하는 로직과 OTA image writing 로직이 필요하다.
- ESP-IDF bootloader가 OTA 기능을 포함하는지도 확인해야 한다.
- partition table offset은 기본적으로 `0x8000`을 전제로 한다. 변경할 경우 bootloader와 flashing command 양쪽에서 같은 값을 사용해야 한다.
