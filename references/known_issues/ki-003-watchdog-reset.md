# KI-003: Watchdog May Reset the Chip After App Entry

Status: Open

## Summary

The current app never disables or feeds watchdogs. It enters a tight loop that repeatedly reads GPIO and writes UART0 FIFO.

```rust
loop {
    let gpio_in_val = dp.gpio.in_().read().bits();
    ...
}
```

## Risk

ESP-IDF bootloader or ROM may leave watchdogs enabled. A bare `riscv-rt` app does not automatically disable ESP32-P4 watchdogs.

Potential symptoms:

- app appears to run briefly
- no app output is visible
- chip resets after a fixed delay
- reset reason may point to watchdog or system reset depending on stage

## Current evidence

The user observed that after bootloader handoff the chip appears to reset after some time. No app-level log is visible, so watchdog reset is a plausible cause.

## Next checks

1. Record the reset reason after the delayed reset.
2. Disable or feed watchdogs early in `main()`.
3. Identify ESP32-P4 watchdog registers for:
   - TIMG0 WDT
   - TIMG1 WDT
   - RTC/LP watchdog if applicable
4. Compare with ESP-IDF startup code or esp-hal initialization.

## Mitigation options

- Add a minimal `disable_watchdogs()` function at the beginning of `main()`.
- Feed watchdogs in the main loop until full init is implemented.
- Replace `panic_halt` with a panic handler that prints or toggles GPIO before halting.
