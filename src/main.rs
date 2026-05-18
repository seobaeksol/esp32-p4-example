#![no_std]
#![no_main]

// include panic handler
use panic_halt as _;

// esp-riscv-rt macros
use esp_riscv_rt::entry;

// bare-metal helper function to directly push a string into UART0 FIFO register
fn print_uart0(uart0: &pac::uart0::RegisterBlock, msg: &str) {
    for &byte in msg.as_bytes() {
        while uart0.status().read().txfifo_cnt().bits() >= 127 {
            // It the buffer is full, wait until it becomes empty
            unsafe { core::arch::asm!("nop") };
        }

        uart0.fifo().write(|w| unsafe { w.bits(byte as u32) });
    }
}

#[entry]
fn main() -> ! {
    let uart0 = unsafe { &pac::UART0::steal() };

    print_uart0(uart0, "ESP32-P4 Real Bare-metal Started!\r\n");
    print_uart0(uart0, "ESP32-P4 Real Bare-metal Started!\r\n");
    loop {}
}
