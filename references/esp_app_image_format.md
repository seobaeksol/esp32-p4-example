# Current ESP-IDF App Image Notes

Last updated: 2026-05-21

This project is a Rust `no_std` bare-metal app for ESP32-P4. It does not use ESP-IDF as the application framework, but it is flashed as an ESP-IDF bootloader compatible application image.

## Current boot flow

```text
ESP-ROM -> ESP-IDF v5.5.3 2nd stage bootloader -> Rust app
```

The app is built from:

- `riscv-rt = 0.17.1`
- local PAC in `pac/`
- `panic-halt`
- manually defined `esp_app_desc`

`esp-riscv-rt` is no longer in the active dependency set.

## App descriptor

ESP-IDF tooling/bootloader expects an application descriptor symbol named `esp_app_desc`.

Current implementation:

- file: `src/esp_app_desc.rs`
- symbol: `esp_app_desc`
- section: `.flash.appdesc`
- magic: `0xABCD5432`
- size: 256 bytes

Important fields:

```text
min_efuse_blk_rev_full = 0
max_efuse_blk_rev_full = 65535
mmu_page_size = 16  # log2(64 KiB)
```

Historical note: when the bootloader read code bytes as the descriptor, it reported an incorrect eFuse block revision requirement. The current simplified layout no longer shows that rejection.

## Current linker/image behavior

Active linker scripts:

```text
memory.x
riscv-rt generated link.x
```

The previously experimental `esp-idf-sections.x` is no longer active.

Current `espflash save-image` shape is accepted by the bootloader but still noncanonical:

```text
segments = 4
entry = 0x40000020

segment 0: vaddr=0x00000000
segment 1: vaddr=0x40000614 map
segment 2: vaddr=0x00000000
segment 3: vaddr=0x40000020 map
```

See:

- `references/known_issues/ki-005-noncanonical-esp-image-segments.md`

## Flash command

Use the cargo runner:

```bash
cargo run --release -- -M
```

Equivalent flash options:

```bash
espflash flash \
  --monitor \
  --chip esp32p4 \
  --flash-size 16mb \
  --flash-mode qio \
  --flash-freq 80mhz \
  --partition-table partitions.csv \
  target/riscv32imafc-unknown-none-elf/release/esp32-p4-example
```

## Current status

The image now reaches app handoff:

```text
I boot: Loaded app from partition at offset 0x10000
I boot: Disabling RNG early entropy source...
```

No app UART output is confirmed yet, and delayed reset is suspected. Current investigation has moved to runtime issues:

- output path mismatch
- watchdog
- RAM/L2 cache overlap
- missing ESP32-P4-specific runtime initialization

See `references/current_status.md` and `references/known_issues/`.
