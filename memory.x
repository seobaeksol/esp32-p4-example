/* ESP32-P4 / JC4880P443C_I_W memory layout for esp-riscv-rt.
 *
 * Board/module facts:
 * - External flash: 16 MB
 * - External PSRAM: 32 MB
 * - HP L2MEM: 768 KB, main internal RAM
 * - HP SPM: 8 KB scratchpad
 * - LP SRAM: 32 KB
 *
 * Address ranges are from ESP32-P4 TRM Chapter 7.
 */

MEMORY
{
    /* External flash virtual address window. Actual module flash size: 16 MB. */
    FLASH  : ORIGIN = 0x40000000, LENGTH = 16M

    /* HP L2MEM, usable as the primary internal RAM for data/bss/stack. */
    RAM    : ORIGIN = 0x4FF00000, LENGTH = 768K

    /* HP scratchpad memory. Keep separate unless code explicitly places sections here. */
    SPM    : ORIGIN = 0x30100000, LENGTH = 8K

    /* External PSRAM virtual address window. Requires boot/cache/MMU setup before use. */
    PSRAM  : ORIGIN = 0x48000000, LENGTH = 32M

    /* LP SRAM. Keep separate unless LP/retention use is explicitly needed. */
    LP_RAM : ORIGIN = 0x50108000, LENGTH = 32K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

/* ESP32-P4 has two HP CPU harts. This reserves stack space per hart in REGION_STACK. */
_max_hart_id = 1;
_hart_stack_size = 8K;

/* No Rust heap is used by this project yet. Increase when adding an allocator. */
_heap_size = 0;

/* esp-riscv-rt uses this symbol in its pre-default trap path. */
PROVIDE(_dram_data_start = ORIGIN(RAM));
