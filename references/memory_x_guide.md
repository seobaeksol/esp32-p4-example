# ESP32-P4 `memory.x` 작성 가이드

대상 보드/모듈: JC4880P443C_I_W / JC-ESP32P4-M3  
런타임: `esp-riscv-rt`  
실행 방식: `espflash flash`  
참고 자료:

- `references/memory_organization.md`
- `references/chip_boot_control.md`
- `references/JC4880P443_V1.0_schematic.md`

## 1. 보드 메모리 조건

| 항목 | 값 |
|---|---:|
| External Flash | 16 MB |
| External PSRAM | 32 MB |
| HP L2MEM | 768 KB |
| HP SPM | 8 KB |
| LP SRAM | 32 KB |
| HP ROM | 128 KB |

## 2. ESP32-P4 주요 주소 범위

| 메모리 | 주소 범위 | 크기 | 용도 |
|---|---:|---:|---|
| External Flash | `0x4000_0000` ~ `0x43FF_FFFF` | 최대 64 MB window | `.text`, `.rodata` 후보 |
| External PSRAM | `0x4800_0000` ~ `0x4BFF_FFFF` | 최대 64 MB window | 큰 heap/buffer 후보 |
| HP L2MEM | `0x4FF0_0000` ~ `0x4FFB_FFFF` | 768 KB | 주 RAM 후보 |
| HP SPM | `0x3010_0000` ~ `0x3010_1FFF` | 8 KB | scratchpad |
| LP SRAM | `0x5010_8000` ~ `0x5010_FFFF` | 32 KB | LP/retention 용도 |

이 보드는 Flash 16 MB, PSRAM 32 MB이므로 `memory.x`에서는 window 전체가 아니라 실제 탑재 크기만 잡는다.

## 3. `riscv-rt` / `esp-riscv-rt`에서 필요한 region alias

`riscv-rt`의 `link.x`는 아래 alias를 사용한다.

```ld
REGION_ALIAS("REGION_TEXT", ...);
REGION_ALIAS("REGION_RODATA", ...);
REGION_ALIAS("REGION_DATA", ...);
REGION_ALIAS("REGION_BSS", ...);
REGION_ALIAS("REGION_HEAP", ...);
REGION_ALIAS("REGION_STACK", ...);
```

기본 권장 배치는 다음과 같다.

| Section | 추천 region | 이유 |
|---|---|---|
| `.text` | `FLASH` | 코드 저장 영역 |
| `.rodata` | `FLASH` | 읽기 전용 상수 저장 영역 |
| `.data` | `RAM` / HP L2MEM | 런타임에 쓰기 가능해야 함 |
| `.bss` | `RAM` / HP L2MEM | zero-init 데이터 |
| heap | `RAM` / HP L2MEM | 초기에는 내부 RAM 사용 권장 |
| stack | `RAM` / HP L2MEM | 가장 안전한 기본 stack 위치 |

PSRAM은 cache/MMU/부트 초기화가 필요할 수 있으므로, 초기 bring-up 단계에서는 기본 `.data`, `.bss`, stack에 바로 쓰지 않는 편이 안전하다.

## 4. 현재 프로젝트용 `memory.x`

현재 프로젝트의 `memory.x`는 다음 정책을 사용한다.

```ld
MEMORY
{
    FLASH  : ORIGIN = 0x40000000, LENGTH = 16M
    RAM    : ORIGIN = 0x4FF00000, LENGTH = 768K
    SPM    : ORIGIN = 0x30100000, LENGTH = 8K
    PSRAM  : ORIGIN = 0x48000000, LENGTH = 32M
    LP_RAM : ORIGIN = 0x50108000, LENGTH = 32K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

_max_hart_id = 1;
_hart_stack_size = 8K;
_heap_size = 0;

PROVIDE(_dram_data_start = ORIGIN(RAM));
```

### 설정 설명

| 설정 | 의미 |
|---|---|
| `FLASH = 0x40000000, 16M` | 보드의 16 MB flash를 external flash virtual address에 배치 |
| `RAM = 0x4FF00000, 768K` | HP L2MEM을 주 RAM으로 사용 |
| `PSRAM = 0x48000000, 32M` | 보드의 32 MB PSRAM window 정의. 기본 section에는 아직 미사용 |
| `_max_hart_id = 1` | HP dual-core를 고려한 hart id 범위 |
| `_hart_stack_size = 8K` | hart별 8 KB stack 예약 |
| `_heap_size = 0` | 현재 allocator가 없으므로 heap 비활성 |
| `_dram_data_start` | `esp-riscv-rt` trap path에서 참조하는 RAM 시작 심볼 |

## 5. `.cargo/config.toml` 주의사항

`rustflag`가 아니라 반드시 `rustflags`여야 한다.

권장 설정:

```toml
[target.riscv32imafc-unknown-none-elf]
runner = "espflash flash --monitor"
rustflags = [
    "-C", "link-arg=-Tmemory.x",
    "-C", "link-arg=-Tlink.x",
]

[build]
target = "riscv32imafc-unknown-none-elf"
```

`memory.x`는 `link.x`보다 먼저 전달되어야 한다.

## 6. `esp-riscv-rt` 직접 사용 시 추가로 필요한 것

`esp-hal` 없이 `esp-riscv-rt`만 직접 쓰면 HAL이 제공하는 일부 hook이 없다. 현재 프로젝트는 빌드를 위해 최소 stub을 제공한다.

필요했던 심볼:

| 심볼 | 처리 |
|---|---|
| `handle_interrupts` | `src/main.rs`에서 halt loop로 제공 |
| `_start_trap_rust_hal` | `src/main.rs`에서 halt loop로 제공 |
| `_dram_data_start` | `memory.x`에서 `PROVIDE()` |
| critical-section acquire/release | `riscv` crate의 `critical-section-single-hart` feature 사용 |

`Cargo.toml`:

```toml
riscv = { version = "0.16.0", features = ["critical-section-single-hart"] }
```

> 주의: `critical-section-single-hart`는 단일 hart 전제의 critical section 구현이다. 멀티코어를 실제로 활성화해서 공유 데이터를 다룰 경우 더 적절한 critical-section 구현을 검토해야 한다.

## 7. 초기 bring-up 권장 순서

1. `memory.x`에서 RAM은 HP L2MEM만 기본 사용한다.
2. PSRAM은 region만 정의하고 기본 section에는 배치하지 않는다.
3. stack/heap 크기를 작게 시작한다.
4. UART 출력 같은 최소 기능으로 부팅 확인한다.
5. 이후 flash image format, bootloader 흐름, cache/MMU 초기화가 확인되면 PSRAM heap 또는 큰 buffer section을 추가한다.

## 8. 문제 해결 체크리스트

### `unused config key build.rustflag`

원인: `.cargo/config.toml` 오타.

해결:

```toml
rustflags = [...]
```

### `_stack_start`, `__sdata`, `__ebss`, `hal_main` undefined

원인: `memory.x` / `link.x`가 링커에 전달되지 않음.

해결:

```toml
rustflags = [
    "-C", "link-arg=-Tmemory.x",
    "-C", "link-arg=-Tlink.x",
]
```

### `REGION_DATA` 또는 `RAM` 관련 오류

원인: `REGION_ALIAS(..., RAM)`를 쓰지만 `MEMORY`에 `RAM` region이 없음.

해결: HP L2MEM을 `RAM`으로 정의.

```ld
RAM : ORIGIN = 0x4FF00000, LENGTH = 768K
```

### `handle_interrupts`, `_start_trap_rust_hal` undefined

원인: `esp-riscv-rt`를 HAL 없이 직접 사용.

해결: 최소 hook을 직접 제공하거나 `esp-hal` 사용을 검토.

## 9. 현재 빌드 상태

아래 명령으로 빌드 성공 확인됨.

```bash
cargo build
```

결과:

```text
Finished `dev` profile [unoptimized + debuginfo]
```
