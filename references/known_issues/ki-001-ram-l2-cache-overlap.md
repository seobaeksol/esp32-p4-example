# KI-001: RAM Origin May Overlap ESP-IDF L2 Cache Reservation

Status: Open

## Summary

Current `memory.x` uses:

```ld
RAM : ORIGIN = 0x4ff00000, LENGTH = 64K
```

This is simple and currently lets the image pass the bootloader, but it may be unsafe in the ESP-IDF 2nd stage bootloader flow.

## Risk

ESP32-P4 HP L2MEM is shared with L2 cache. ESP-IDF bootloader may reserve the low part of HP L2MEM for cache. Earlier `esp-hal` analysis suggested the IDF default 128 KiB L2 cache layout leaves app RAM starting around:

```ld
RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8E000
```

Using `0x4ff00000` can overlap cache-reserved memory, causing undefined behavior after app entry.

Potential symptoms:

- app hangs without output
- delayed reset
- random instruction/data faults
- corrupted stack/data

## Current evidence

The app reaches bootloader handoff, but no app-level UART output is confirmed and reset may occur later.

The current stack is placed in the 64 KiB region starting at `0x4ff00000`, so stack/cache conflict is plausible.

## Next checks

1. Try `RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8E000` with current simple image layout.
2. Compare boot behavior and reset timing.
3. Inspect ESP-IDF bootloader cache configuration for this board/build.
4. Confirm whether `0x4ff00000..0x4ff1ffff` is reserved at app entry.

## Mitigation options

- Move RAM origin to `0x4FF20000`.
- Keep stack small and away from low L2MEM until cache ownership is confirmed.
- Adopt the ESP32-P4 `esp-hal` RAM layout more closely.
