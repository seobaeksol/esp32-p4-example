# Current `memory.x` Guide

Last updated: 2026-05-21

This document describes the active `memory.x` used by the current codebase.

## Active file

```ld
MEMORY
{
  RAM : ORIGIN = 0x4ff00000, LENGTH = 64K
  FLASH : ORIGIN = 0x40000020, LENGTH = 1M
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

SECTIONS {
  .text_gap (NOLOAD): {
    . = . + 4;
    . = ALIGN(4) + 0x20;
  } > FLASH
}
INSERT BEFORE .rodata;
```

## Why this simple layout is currently used

Earlier experiments tried to build a more canonical ESP-IDF DROM/IROM layout. Those experiments fixed some bootloader validation problems but later hit instruction fetch/runtime issues.

The current simplified layout is kept because it reaches app handoff:

```text
I boot: Loaded app from partition at offset 0x10000
I boot: Disabling RNG early entropy source...
```

The current priority is now app runtime validation, not further image-layout cleanup.

## FLASH origin

```ld
FLASH : ORIGIN = 0x40000020, LENGTH = 1M
```

`0x40000020` skips the ESP application image header area and keeps mapped segment address low bits compatible with a factory partition at `0x10000`.

The `1M` size matches the current `factory` partition size in `partitions.csv`.

## RAM origin

```ld
RAM : ORIGIN = 0x4ff00000, LENGTH = 64K
```

This is intentionally minimal, but risky. It may overlap ESP-IDF bootloader L2 cache reservation. See:

- `references/known_issues/ki-001-ram-l2-cache-overlap.md`

## `.text_gap`

The inserted `.text_gap` is used to encourage `espflash` to produce separate mapped portions instead of one fully merged section.

```ld
SECTIONS {
  .text_gap (NOLOAD): {
    . = . + 4;
    . = ALIGN(4) + 0x20;
  } > FLASH
}
INSERT BEFORE .rodata;
```

This is a pragmatic compatibility tweak, not a final canonical layout.

## Current limitations

Known risks:

- RAM may overlap L2 cache reservation.
- ESP image still contains dummy `vaddr=0` segments.
- UART output is not confirmed.
- Watchdog handling is not implemented.

See `references/known_issues/`.
