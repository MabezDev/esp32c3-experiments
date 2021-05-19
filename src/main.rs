#![no_std]
#![no_main]
#![feature(llvm_asm)]

use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_timer_CountDown};
use panic_halt as _;

use core::fmt::Write;
use riscv_rt::entry;

use esp32c3_lib::{EtsTimer, GpioOutput, Uart, disable_wdts};

#[entry]
fn main() -> ! {
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe {
        let mut _tmp: u32;
        llvm_asm!("csrrs $0, mstatus, $1": "=r"(_tmp) : "rK"(0x00000008))
    };

    // disable wdt's
    disable_wdts();

    let mut gpio18 = GpioOutput::new(18);

    writeln!(Uart, "Hello world!").unwrap();

    let mut delay = EtsTimer::new(1_000_000);

    loop {
        gpio18.set_high().unwrap();
        nb::block!(delay.wait()).unwrap();
        gpio18.set_low().unwrap();
        nb::block!(delay.wait()).unwrap();
    }
}
