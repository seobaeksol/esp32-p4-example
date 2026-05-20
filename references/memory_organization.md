# ESP32-P4 Memory Organization

Source: `references/memory_organization.txt`  
TRM section: Part III, Chapter 7 — System and Memory

## 1. Overview

ESP32-P4 integrates two RISC-V processors:

| Processor | Description | Max clock |
|---|---|---:|
| HP CPU | High-performance 32-bit RISC-V dual-core processor, five-stage pipeline | 360 MHz |
| LP CPU | Low-power 32-bit RISC-V single-core processor, two-stage pipeline | 40 MHz |

All internal memory, external memory, and peripherals are located on the HP CPU and LP CPU buses.

Both HP CPU and LP CPU use little-endian instruction/data buses. HP CPU DBUS is 128-bit wide; other buses are 32-bit wide.

## 2. Main Features

### Address Space

| Region type | Size |
|---|---:|
| HP internal memory address space, cache-accessible by HP CPU instruction/data bus | 896 KB |
| HP internal memory address space, directly accessible by HP CPU instruction/data bus | 8 KB |
| LP internal memory address space, directly accessible by LP CPU instruction/data bus | 48 KB |
| Peripheral address space | 1256 KB |
| External flash virtual address space | 64 MB |
| External RAM virtual address space | 64 MB |
| Internal DMA address space | 768 KB |
| External DMA address space | 128 MB |

### Internal Memory

| Memory | Size | Notes |
|---|---:|---|
| HP ROM | 128 KB | Read-only, HP system ROM code/data |
| HP L2MEM | 768 KB | Volatile RAM, accessible by HP CPU, LP CPU, DMA |
| HP SPM | 8 KB | Scratchpad memory, direct HP CPU access |
| LP ROM | 16 KB | Read-only, LP system boot/basic functions |
| LP SRAM | 32 KB | Volatile RAM, accessible by HP CPU and LP CPU |

### External Memory

ESP32-P4 supports up to:

| External memory | Max size |
|---|---:|
| External flash | 64 MB |
| External RAM | 64 MB |

Supported interfaces:

- External flash: SPI, Dual SPI, Quad SPI, QPI
- External RAM: OPI, HPI

## 3. CPU Access Capabilities

### HP CPU

HP CPU can:

- Directly access HP SPM through both instruction bus and data bus.
- Access internal memory and external memory through cache:
  - HP ROM
  - HP L2MEM
  - External flash
  - External RAM
- Directly access some internal/external memory without cache through `0x8xxx_xxxx` ranges.
- Access HP CPU peripherals, HP peripherals, and LP peripherals through data bus.

Address ranges starting with `0x4xxx_xxxx` can be configured as cacheable or non-cacheable depending on CPU PMA. Address ranges starting with `0x8xxx_xxxx` are direct CPU access and are generally slower, often used for debugging.

### LP CPU

LP CPU can:

- Directly access LP SRAM and LP ROM through instruction/data bus.
- Access HP ROM, HP L2MEM, external flash, and external RAM.
- Access HP CPU peripherals, HP peripherals, and LP peripherals through data bus.

## 4. System Memory Address Mapping

| Bus type | Low address | High address | Size | Target |
|---|---:|---:|---:|---|
| Reserved | `0x0000_0000` | `0x300F_FFFF` | - | Reserved |
| Data/Instruction | `0x3010_0000` | `0x3010_1FFF` | 8 KB | HP SPM |
| Reserved | `0x3010_2000` | `0x3FEF_FFFF` | - | Reserved |
| Data/Instruction | `0x3FF0_0000` | `0x3FF1_FFFF` | 128 KB | HP CPU peripherals |
| Reserved | `0x3FF2_0000` | `0x3FFF_FFFF` | - | Reserved |
| Data/Instruction | `0x4000_0000` | `0x43FF_FFFF` | 64 MB | External flash |
| Reserved | `0x4400_0000` | `0x47FF_FFFF` | - | Reserved |
| Data/Instruction | `0x4800_0000` | `0x4BFF_FFFF` | 64 MB | External RAM |
| Reserved | `0x4C00_0000` | `0x4FBF_FFFF` | - | Reserved |
| Data/Instruction | `0x4FC0_0000` | `0x4FC1_FFFF` | 128 KB | HP ROM |
| Reserved | `0x4FC2_0000` | `0x4FEF_FFFF` | - | Reserved |
| Data/Instruction | `0x4FF0_0000` | `0x4FFB_FFFF` | 768 KB | HP L2MEM |
| Reserved | `0x4FFC_0000` | `0x4FFF_FFFF` | - | Reserved |
| Data/Instruction | `0x5000_0000` | `0x500F_FFFF` | 1 MB | HP peripherals |
| Data/Instruction | `0x5010_0000` | `0x5010_3FFF` | 16 KB | LP ROM |
| Reserved | `0x5010_4000` | `0x5010_7FFF` | - | Reserved / MIPI internal memory subranges |
| Data/Instruction | `0x5010_8000` | `0x5010_FFFF` | 32 KB | LP SRAM |
| Data/Instruction | `0x5011_0000` | `0x5012_FFFF` | 128 KB | LP peripherals |
| Reserved | `0x5013_0000` | `0x7FFF_FFFF` | - | Reserved |
| Data/Instruction | `0x8000_0000` | `0x83FF_FFFF` | 64 MB | External flash, direct CPU access |
| Reserved | `0x8400_0000` | `0x87FF_FFFF` | - | Reserved |
| Data/Instruction | `0x8800_0000` | `0x8BFF_FFFF` | 64 MB | External RAM, direct CPU access |
| Reserved | `0x8C00_0000` | `0x8FBF_FFFF` | - | Reserved |
| Data/Instruction | `0x8FC0_0000` | `0x8FC1_FFFF` | 128 KB | HP ROM, direct CPU access |
| Reserved | `0x8FC2_0000` | `0x8FEF_FFFF` | - | Reserved |
| Data/Instruction | `0x8FF0_0000` | `0x8FFB_FFFF` | 768 KB | HP L2MEM, direct CPU access |
| Reserved | `0x8FFC_0000` | `0xFFFF_FFFF` | - | Reserved |

## 5. Internal Memory Details

### HP ROM

- Size: 128 KB
- Read-only memory for HP system ROM code and read-only data.
- Accessed by HP CPU through instruction/data bus via L2 cache.

Address ranges:

| Access type | Range |
|---|---|
| Cacheable or non-cacheable, PMA-dependent | `0x4FC0_0000` ~ `0x4FC1_FFFF` |
| Direct CPU access | `0x8FC0_0000` ~ `0x8FC1_FFFF` |

### HP L2MEM

- Size: 768 KB
- Read/write volatile memory.
- Accessible by HP CPU, LP CPU, and DMA peripherals.
- Operates at half HP CPU frequency.
- Can be configured to retain power during Light-sleep.
- Supports ECC check.
- Consists of six 128 KB units.

Address ranges:

| Access type | Range |
|---|---|
| Cacheable or non-cacheable, PMA-dependent | `0x4FF0_0000` ~ `0x4FFB_FFFF` |
| Direct CPU access | `0x8FF0_0000` ~ `0x8FFB_FFFF` |

Important note: HP L2MEM must be initialized before ECC checking. Avoid accessing an L2MEM unit while it is being initialized.

### LP ROM

- Size: 16 KB
- Read-only memory for LP CPU boot code and basic system functions.
- Address range: `0x5010_0000` ~ `0x5010_3FFF`

### LP SRAM

- Size: 32 KB
- Read/write volatile memory.
- Accessible by HP CPU through AHB matrix and by LP CPU through instruction/data bus.
- Supports atomic operations.
- Address range: `0x5010_8000` ~ `0x5010_FFFF`

### HP SPM

- Size: 8 KB
- Scratchpad memory.
- Read/write volatile memory.
- Directly accessed by HP CPU through instruction/data bus.
- One access completes in two cycles.
- Supports HP CPU atomic operations.
- Supports parity check.
- Address range: `0x3010_0000` ~ `0x3010_1FFF`

Important note: HP SPM must be initialized before parity checking. Avoid accessing HP SPM during initialization.

## 6. External Memory Mapping

HP CPU can access external flash and RAM through either cacheable/PMA-controlled ranges or direct CPU access ranges.

| Memory | Cached/PMA-dependent range | Direct CPU access range |
|---|---|---|
| External flash | `0x4000_0000` ~ `0x43FF_FFFF` | `0x8000_0000` ~ `0x83FF_FFFF` |
| External RAM | `0x4800_0000` ~ `0x4BFF_FFFF` | `0x8800_0000` ~ `0x8BFF_FFFF` |

The MMU maps virtual addresses to physical addresses in external memory.

Security features for external memory:

- External flash: hardware manual encryption and automatic decryption using XTS-AES.
- External RAM: automatic encryption/decryption using XTS-AES.

## 7. Cache

ESP32-P4 has a two-level cache system.

| Cache | Size / feature |
|---|---|
| L1 instruction cache | 16 KB, 64 B block, 4-way set associative |
| L1 data cache | 64 KB, 64 B block, 2-way set associative, write-through/write-back |
| L2 cache | 128 KB / 256 KB / 512 KB, 64 B / 128 B block, 8-way set associative |

Supported cache operations:

- Write-back
- Clean
- Invalidate
- Preload
- Lock / Unlock

Notes:

- Write-back applies to dcache and L2 cache.
- Clean clears dirty bits without updating external memory.
- Invalidate removes valid data from cache.
- Manual invalidate works only on unlocked cache data.

## 8. DMA Address Space

ESP32-P4 includes:

- GDMA-AHB
- GDMA-AXI
- VDMA

DMA-capable masters can access:

| Memory | Range |
|---|---|
| HP L2MEM | `0x4FF0_0000` ~ `0x4FFB_FFFF` |
| External flash | `0x4000_0000` ~ `0x43FF_FFFF` |
| External RAM | `0x4800_0000` ~ `0x4BFF_FFFF` |

VDMA Master 1 can additionally access:

| Memory | Range |
|---|---|
| MIPI CSI internal memory | `0x5010_4000` ~ `0x5010_4FFF` |
| MIPI DSI internal memory | `0x5010_5000` ~ `0x5010_5FFF` |

DMA access requires corresponding permission configuration; otherwise, access may fail.

## 9. Peripheral Address Summary

### HP CPU Peripherals

| Target | Range | Size |
|---|---|---:|
| HP CPU peripherals | `0x3FF0_0000` ~ `0x3FF1_FFFF` | 128 KB |

Examples include:

- RISC-V Trace Encoder 0/1
- Bus Monitor
- L2MEM Monitor
- SPM Monitor

### HP Peripherals

| Target | Range | Size |
|---|---|---:|
| HP peripherals | `0x5000_0000` ~ `0x500F_FFFF` | 1 MB |

Examples include:

- USB 2.0 OTG HS/FS
- VDMA
- SD/MMC
- H264 Encoder
- GDMA-AHB / GDMA-AXI
- JPEG
- PPA
- AES / SHA / RSA / HMAC / ECC
- GMAC
- CSI / DSI Host
- ISP
- RMT
- MCPWM
- TIMG
- I2C
- I2S
- UART0~UART4
- SPI2 / SPI3
- USB Serial/JTAG
- LEDC
- Interrupt Matrix
- LCD_CAM
- ADC
- GPIO
- IO MUX
- SYSTIMER
- Reset and Clock

### LP Peripherals

| Target | Range | Size |
|---|---|---:|
| LP peripherals | `0x5011_0000` ~ `0x5012_FFFF` | 128 KB |

Examples include:

- LP System Register
- LP Always-on Clock and Reset
- LP Timer
- PMU
- LP Watchdog Timer
- LP Mailbox
- LP UART
- LP I2C
- LP SPI
- LP I2S
- LP ADC
- LP GPIO Matrix
- LP IO MUX
- LP Interrupt
- LP eFuse
- LP Temperature Sensor

## 10. Notes for `memory.x` Planning

For a bare-metal Rust linker script, useful RAM candidates are:

| Candidate | Address | Size | Notes |
|---|---:|---:|---|
| HP SPM | `0x3010_0000` | 8 KB | Very small scratchpad; direct HP CPU access |
| HP L2MEM | `0x4FF0_0000` | 768 KB | Main internal RAM candidate |
| HP L2MEM direct alias | `0x8FF0_0000` | 768 KB | Direct CPU access alias; slower/debug-oriented |
| External RAM | `0x4800_0000` | up to 64 MB | Requires external RAM/MMU/cache setup |
| LP SRAM | `0x5010_8000` | 32 KB | Small LP-accessible RAM |

For initial bare-metal experiments, HP L2MEM is the most natural internal RAM region for `.data`, `.bss`, heap, and stack, but boot/runtime initialization requirements and ESP boot flow must be considered.
