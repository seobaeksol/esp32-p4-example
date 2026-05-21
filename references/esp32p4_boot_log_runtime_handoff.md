# Boot Log Analysis: Runtime Handoff Reached

Last updated: 2026-05-21

## Summary

The current simplified `riscv-rt` + `memory.x` setup reaches app handoff.

Observed final bootloader lines:

```text
I boot: Loaded app from partition at offset 0x10000
I boot: Disabling RNG early entropy source...
```

No immediate bootloader assert or Guru Meditation is shown in this log.

## Current image segments from boot log

```text
segment 0: paddr=00010020 vaddr=00000000 size=005ech
segment 1: paddr=00010614 vaddr=40000614 size=0029ch map
segment 2: paddr=000108b8 vaddr=00000000 size=0f760h
segment 3: paddr=00020020 vaddr=40000020 size=005d0h map
```

This segment layout is not canonical, but the bootloader accepts it.

## Interpretation

Current blocker has moved from image compatibility to runtime behavior.

Likely runtime issues:

1. UART output path mismatch: app writes UART0 FIFO but monitor may be USB Serial/JTAG CDC.
2. Watchdogs are not disabled/fed.
3. Current RAM origin `0x4ff00000` may overlap L2 cache reservation.
4. Generic `riscv-rt` lacks ESP32-P4-specific init.

See `references/known_issues/` for individual issue tracking.

## Immediate next debugging steps

1. Add an early GPIO toggle to prove `main()` executes without relying on UART.
2. Confirm output path: UART0 pins vs USB Serial/JTAG.
3. Disable/feed watchdogs early.
4. Test safer RAM origin `0x4FF20000` after adding a visible liveness signal.
