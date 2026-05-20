#![no_std]
#![no_main]

// include panic handler
use panic_halt as _;

// esp-riscv-rt macros
use esp_riscv_rt::{TrapFrame, entry};

// Minimal trap/interrupt hooks normally provided by a HAL such as esp-hal.
// This project currently uses esp-riscv-rt directly, so provide simple halt loops.
#[unsafe(no_mangle)]
extern "C" fn handle_interrupts() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

#[unsafe(no_mangle)]
extern "C" fn _start_trap_rust_hal(_trap_frame: &TrapFrame) {
    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

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
    let dp = pac::Peripherals::take().unwrap();
    let uart0 = &dp.uart0;

    print_uart0(uart0, "ESP32-P4 Real Bare-metal Started!\r\n");
    print_uart0(uart0, "Reading GPIO5 status\r\n");

    loop {
        let gpio_in_val = dp.gpio.in_().read().bits();
        let pin5_state = (gpio_in_val >> 5) & 1;

        if pin5_state == 1 {
            print_uart0(uart0, "Hello World! (GPIO5 is HIGH)\r\n");
        } else {
            print_uart0(uart0, "... (GPIO is LOW)");
        }
    }
}
