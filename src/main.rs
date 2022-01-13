#![no_std]
#![no_main]
#![feature(asm_const)]

use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_timer_CountDown};
use panic_halt as _;

use core::{arch::asm, fmt::Write};
use riscv_rt::entry;

use esp32c3_lib::{disable_wdts, EtsTimer, GpioOutput, Uart};

// make sure we have something in our rodata section
#[used]
static RODATA_SECTION_TEST: &'static str = "TEST DATA";
// make sure we have something in our data section
#[used]
static mut DATA_SECTION_TEST: [u8; 12] = [0xAA; 12];

#[entry]
fn main() -> ! {
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe {
        let mut _tmp: u32;
        asm!("csrrsi {0}, mstatus, {1}", out(reg) _tmp, const 0x00000008)
    };

    // disable wdt's
    disable_wdts();

    let mut gpio = GpioOutput::new(9);

    writeln!(Uart, "Hello world!").unwrap();
    writeln!(Uart, "{}", RODATA_SECTION_TEST).unwrap();
    unsafe {
        writeln!(Uart, "DATA_SECTION_TEST: {:x?}", DATA_SECTION_TEST).unwrap();
    }

    let mut delay = EtsTimer::new(1_000_000);

    loop {
        writeln!(Uart, "HIGH").unwrap();
        gpio.set_high().unwrap();
        nb::block!(delay.wait()).unwrap();

        writeln!(Uart, "LOW").unwrap();
        gpio.set_low().unwrap();
        nb::block!(delay.wait()).unwrap();
    }
}
