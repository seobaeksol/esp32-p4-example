# ESP32-P4 Chip Boot Control

Source: `chip_boot_control.txt`  
TRM section: Chapter 11 — Chip Boot Control

## 1. Overview

ESP32-P4의 부팅 과정과 일부 칩 기능은 **Power-on reset** 또는 **hardware reset** 시점에 다음 값으로 결정된다.

- Strapping pin 상태
- eFuse bit 상태

이 값들로 결정되는 주요 기능은 다음과 같다.

- Chip boot mode
- ROM message 출력 활성화/비활성화
- JTAG signal source

## 2. Strapping Pins

ESP32-P4에는 5개의 strapping pin이 있다.

| Strapping pin |
|---|
| GPIO34 |
| GPIO35 |
| GPIO36 |
| GPIO37 |
| GPIO38 |

Chip Reset 중 하드웨어는 각 strapping pin의 전압 레벨을 샘플링하여 `0` 또는 `1`로 latch에 저장한다. 이 값은 전원이 꺼질 때까지 유지된다.

소프트웨어는 `GPIO_STRAPPING`에서 latch된 strapping 값을 읽을 수 있다.

## 3. Default Strapping Configuration

기본적으로 GPIO35만 내부 pull-up에 연결되어 있다.

| Strapping pin | Default configuration |
|---|---|
| GPIO34 | Floating |
| GPIO35 | Pull-up |
| GPIO36 | Floating |
| GPIO37 | Floating |
| GPIO38 | Floating |

GPIO35가 연결되지 않았거나 외부 high-impedance 회로에 연결되어 있으면 내부 weak pull-up에 의해 기본 입력 레벨이 결정된다.

Strapping bit 값을 바꾸려면 다음 방법을 사용할 수 있다.

- 외부 pull-up / pull-down 저항 사용
- Host MCU GPIO로 power-on 시점의 전압 제어

Reset이 해제된 뒤에는 strapping pin도 일반 GPIO/function pin으로 동작한다.

> 주의: 문서화되지 않은 strapping pattern은 예상하지 못한 동작을 유발할 수 있으므로 사용하면 안 된다.

## 4. Boot Mode Control

Reset 시점의 GPIO35, GPIO36, GPIO37, GPIO38 값이 boot mode를 결정한다.

| Boot mode | GPIO35 | GPIO36 | GPIO37 | GPIO38 |
|---|---:|---:|---:|---:|
| SPI Boot mode, default | 1 | x | x | x |
| Joint Download Boot mode | 0 | 1 | x | x |
| SPI Download Boot mode | 0 | 0 | 0 | 1 |
| Invalid combination | 0 | 0 | 1 | x |
| Invalid combination | 0 | 0 | 0 | 0 |

`x`는 결과에 영향을 주지 않는 don't-care 값이다.

### 4.1 SPI Boot Mode

SPI Boot mode에서는 ROM bootloader가 SPI flash에서 프로그램을 로드하고 실행한다.

SPI Boot mode는 다시 두 방식으로 나뉜다.

| Mode | Description |
|---|---|
| Normal flash Boot | Secure Boot 지원. ROM bootloader가 flash에서 프로그램을 L2MEM으로 로드한 뒤 실행한다. 일반적으로 이 프로그램은 2nd stage bootloader이다. |
| Direct Boot | Secure Boot 미지원. 프로그램이 flash에서 직접 실행된다. 이 모드를 활성화하려면 flash에 다운로드된 bin 파일의 첫 두 word가 `0xaedb041d`여야 한다. |

### 4.2 Joint Download Boot Mode

Joint Download Boot mode에서는 다음 인터페이스로 binary file을 flash에 다운로드할 수 있다.

- USB Serial/JTAG Download Boot
- UART Download Boot
- SPI Slave Download Boot
- USB 2.0 OTG Download Boot

또한 binary file을 L2MEM에 다운로드하고 L2MEM에서 실행할 수도 있다.

### 4.3 SPI Download Boot Mode

SPI Download Boot mode에서는 SPI interface를 통해 binary file을 flash에 다운로드할 수 있다.

또한 binary file을 L2MEM에 다운로드하고 L2MEM에서 실행할 수도 있다.

SPI Download Boot mode를 사용할 때는 GPIO37과 GPIO38을 reserve해야 한다. GPIO37과 GPIO38은 기본적으로 floating이며 reset 시점에는 high-impedance 상태이다.

## 5. Boot Mode 관련 eFuse Bits

다음 eFuse bit들은 boot mode 동작을 제어한다.

| eFuse bit | Description |
|---|---|
| `EFUSE_DIS_FORCE_DOWNLOAD` | 기본값 0. 0이면 소프트웨어가 `LPSYSREG_FORCE_DOWNLOAD_BOOT`를 설정하고 CPU reset을 발생시켜 SPI Boot mode에서 Joint Download Boot mode로 강제 전환할 수 있다. 1이면 이 기능이 비활성화된다. |
| `EFUSE_DIS_DOWNLOAD_MODE` | 1이면 Joint Download Boot mode가 영구적으로 비활성화된다. |
| `EFUSE_ENABLE_SECURITY_DOWNLOAD` | 1이면 Joint Download Boot mode에서 plaintext flash read/write/erase만 허용되고 L2MEM 또는 register operation은 지원되지 않는다. Download Boot mode가 비활성화된 경우 이 eFuse는 무시된다. |
| `EFUSE_DIS_DIRECT_BOOT` | 1이면 SPI Boot mode에서 Direct Boot가 비활성화된다. |
| `EFUSE_DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE` | USB Serial/JTAG Controller가 Joint Download Boot mode로 강제 전환하는 기능을 비활성화한다. |

USB Serial/JTAG Controller도 SPI Boot mode에서 Joint Download Boot mode로, 또는 그 반대로 강제 전환할 수 있다.

## 6. ROM Message Printing Control

초기 SPI Boot 과정에서 ROM code message는 다음 대상으로 출력될 수 있다.

- 기본값: UART0 + USB Serial/JTAG Controller
- UART0 only
- USB Serial/JTAG Controller only

### 6.1 UART0 ROM Printing

`EFUSE_UART_PRINT_CONTROL`과 GPIO36이 UART0로의 ROM message 출력을 제어한다.

| `EFUSE_UART_PRINT_CONTROL` | GPIO36 | ROM code printing to UART0 |
|---:|---:|---|
| 0 | x | Always enabled |
| 1 | 0 | Enabled |
| 1 | 1 | Disabled |
| 2 | 0 | Disabled |
| 2 | 1 | Enabled |
| 3 | x | Always disabled |

`x`는 결과에 영향을 주지 않는 don't-care 값이다.

### 6.2 USB Serial/JTAG ROM Printing

USB Serial/JTAG Controller로의 ROM message 출력은 `EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT`가 제어한다.

| eFuse state | Behavior |
|---|---|
| `EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT = 1` | USB Serial/JTAG로 ROM message 출력 비활성화 |
| `EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT = 0` | USB Serial/JTAG Controller가 활성화되어 있으면 ROM message 출력 가능 |

단, `EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT = 0`이어도 USB Serial/JTAG Controller 자체가 비활성화되어 있으면 USB Serial/JTAG로 message가 출력되지 않는다.

## 7. JTAG Signal Source Control

초기 boot process에서 GPIO34는 JTAG signal source를 제어한다.

관련 제어 값:

- GPIO34
- `EFUSE_DIS_PAD_JTAG`
- `EFUSE_DIS_USB_JTAG`
- `EFUSE_JTAG_SEL_ENABLE`

| `EFUSE_DIS_PAD_JTAG` | `EFUSE_DIS_USB_JTAG` | `EFUSE_JTAG_SEL_ENABLE` | GPIO34 | JTAG signal source |
|---:|---:|---:|---:|---|
| 0 | 0 | 0 | x | USB Serial/JTAG Controller |
| 0 | 0 | 1 | 0 | PAD JTAG pins: MTDI, MTCK, MTMS, MTDO |
| 0 | 0 | 1 | 1 | USB Serial/JTAG Controller |
| 0 | 1 | x | x | PAD JTAG pins: MTDI, MTCK, MTMS, MTDO |
| 1 | 0 | x | x | USB Serial/JTAG Controller |
| 1 | 1 | x | x | JTAG disabled |

`x`는 결과에 영향을 주지 않는 don't-care 값이다.

## 8. Bare-metal / Flashing 관점 요약

ESP32-P4 bare-metal 개발에서 중요한 점은 다음과 같다.

- 일반 실행은 보통 **SPI Boot mode**를 사용한다.
- GPIO35가 기본 pull-up이므로, 별도 조작이 없으면 기본적으로 SPI Boot mode가 선택된다.
- Download mode 진입에는 GPIO35/GPIO36 조합이 중요하다.
- `espflash` 등으로 다운로드/플래시할 때는 보드의 boot/reset 회로가 strapping pin을 올바르게 제어해야 한다.
- Direct Boot는 Secure Boot를 지원하지 않으며, bin image의 첫 두 word 조건이 있다.
- Normal flash Boot에서는 ROM bootloader가 flash의 프로그램을 L2MEM으로 로드해 실행하며, 일반적인 2nd stage bootloader 흐름과 연결된다.
- UART 로그가 보이지 않는 경우 `EFUSE_UART_PRINT_CONTROL`, GPIO36 strapping, USB Serial/JTAG eFuse 상태를 확인해야 한다.
