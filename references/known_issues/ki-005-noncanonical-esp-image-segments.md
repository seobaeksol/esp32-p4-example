# KI-005: Current ESP Image Uses Noncanonical Dummy Segments

Status: Open

## Summary

The current simplified linker setup is accepted by the ESP-IDF bootloader, but `espflash save-image` emits dummy/padding segments with `vaddr=0`.

Observed image shape:

```text
segments = 4
entry = 0x40000020

segment 0: vaddr=0x00000000 size=0x05ec
segment 1: vaddr=0x40000614 size=0x029c map
segment 2: vaddr=0x00000000 size=0xf760
segment 3: vaddr=0x40000020 size=0x05d0 map
```

## Risk

This layout is not canonical compared to ESP-IDF/esp-hal images. Although it currently passes bootloader loading, it may be brittle across:

- optimization levels
- code size changes
- espflash versions
- ESP-IDF bootloader versions
- partition offset changes

## Current evidence

The current bootloader log shows all four segments and then successfully reports:

```text
Loaded app from partition at offset 0x10000
Disabling RNG early entropy source...
```

So this is not the current blocker, but it remains a compatibility risk.

## Next checks

1. Keep this layout while focusing on runtime visibility/reset.
2. Once runtime is confirmed, revisit cleaner DROM/IROM section layout.
3. Compare with manufacturer image and esp-hal generated image.
4. Add a reproducible image inspection script to CI/local checks.

## Mitigation options

- Use a more canonical ESP-IDF-compatible linker layout.
- Ensure app descriptor placement remains valid.
- Avoid relying on accidental espflash padding behavior.
