# KI-002: UART0 Output May Not Be Visible on Monitor

Status: Open

## Summary

The app writes directly to UART0 FIFO:

```rust
uart0.fifo().write(|w| unsafe { w.bits(byte as u32) });
```

The monitor is attached through `/dev/ttyACM0`. On this board this may be USB Serial/JTAG CDC, not UART0 TX/RX.

## Risk

The app may be running, but output is invisible because it goes to a different physical/logical console.

Potential causes:

- UART0 TX pin is not connected to the USB CDC interface.
- UART0 pin mux is not configured by the app.
- UART0 baudrate/clock is only incidentally configured by bootloader and not guaranteed.
- `/dev/ttyACM0` receives ROM/bootloader logs through USB Serial/JTAG, while app writes UART0.

## Current evidence

Bootloader logs are visible, but app messages are not:

```text
ESP32-P4 Real Bare-metal Started!
```

is not observed.

## Next checks

1. Verify board schematic for UART0 TX/RX connection.
2. Attach a USB-UART adapter to the actual UART0 TX pin if available.
3. Implement USB Serial/JTAG output instead of UART0 FIFO.
4. Explicitly configure UART0 clock, baudrate, and IO mux before writing FIFO.
5. Add a GPIO toggle early in `main()` as an output-independent liveness signal.

## Mitigation options

- Use the same console peripheral as ESP-IDF bootloader output.
- Add a minimal UART0 init routine.
- Add GPIO heartbeat for runtime confirmation.
