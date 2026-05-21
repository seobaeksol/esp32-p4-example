# KI-004: Minimal `riscv-rt` Runtime Does Not Perform ESP32-P4-Specific Init

Status: Open

## Summary

The app now uses upstream `riscv-rt`, not `esp-riscv-rt` or `esp-hal`.

`riscv-rt` provides generic RISC-V startup behavior, but it does not know about ESP32-P4 bootloader handoff requirements.

## Risk

After ESP-IDF bootloader jumps to the app, some chip-specific setup may still be required before normal peripheral access is safe.

Examples:

- watchdog state
- interrupt controller / CLIC setup
- trap vector details
- cache/MMU assumptions
- peripheral clock gating
- UART/USB console setup
- multicore/hart state

## Current evidence

The app reaches handoff but does not produce visible output. It may be running with incomplete platform initialization.

Current trap hooks halt silently:

```rust
extern "C" fn handle_interrupts() -> ! { loop { wfi } }
extern "C" fn _start_trap_rust_hal(...) { loop { wfi } }
```

So faults may be hidden unless the ROM/bootloader panic handler catches them.

## Next checks

1. Compare `riscv-rt` startup symbols with ESP32-P4 `esp-riscv-rt`/`esp-hal` expectations.
2. Add explicit trap handler diagnostics.
3. Add early GPIO heartbeat before UART use.
4. Confirm whether secondary hart is parked as expected.
5. Decide whether to keep pure `riscv-rt` or reintroduce selected ESP runtime pieces.

## Mitigation options

- Keep `riscv-rt`, but add explicit ESP32-P4 init functions.
- Use `esp-riscv-rt` once the image layout issues are understood.
- Port only the needed startup pieces from `esp-hal`/ESP-IDF.
