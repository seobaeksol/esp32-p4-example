# Known Issues

This directory tracks risks and unresolved runtime issues for the current ESP32-P4 bare-metal codebase.

Current active issues:

| ID | File | Summary | Status |
|---|---|---|---|
| KI-001 | `ki-001-ram-l2-cache-overlap.md` | Current RAM origin may overlap ESP-IDF bootloader L2 cache reservation | Open |
| KI-002 | `ki-002-uart0-output-not-visible.md` | App writes UART0 FIFO, but monitor may be USB Serial/JTAG CDC | Open |
| KI-003 | `ki-003-watchdog-reset.md` | App does not disable/feed watchdogs, possible delayed reset | Open |
| KI-004 | `ki-004-minimal-runtime-init.md` | `riscv-rt` does not perform ESP32-P4-specific runtime init | Open |
| KI-005 | `ki-005-noncanonical-esp-image-segments.md` | Current accepted image has dummy `vaddr=0` segments | Open |

Resolved historical issues are kept in analysis documents under `references/` rather than this directory.
