#![no_std]
#![no_main]
#![feature(asm)]

use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_timer_CountDown};
use panic_halt as _;

use core::fmt::Write;
use riscv_rt::entry;

use esp32c3_lib::{disable_wdts, EtsTimer, GpioOutput, Uart};

use rtt_target::{rprintln, rtt_init_print};

// make sure we have something in our RO section
#[used]
static RO_SECTION_TEST: &'static str = "TEST DATA";

// make sure we have something in our data section
#[used]
static mut DATA_SECTION_TEST: [u8; 12] = [0xAA; 12];

// make sure we have something in our bss section
static mut BSS_SECTION_TEST: [u8; 12] = [0x0; 12];

#[entry]
fn main() -> ! {
    // disable wdt's
    disable_wdts();

    rtt_init_print!(BlockIfFull);

    let mut gpio = GpioOutput::new(9);

    writeln!(Uart, "Hello world! (RTT INIT'd)").unwrap();
    writeln!(Uart, "RO: {}", RO_SECTION_TEST).unwrap();
    writeln!(Uart, "DATA: {:?}", unsafe { &DATA_SECTION_TEST[..] }).unwrap();
    writeln!(Uart, "BSS: {:?}", unsafe { &BSS_SECTION_TEST[..] }).unwrap();

    let mut delay = EtsTimer::new(1_000_000);
    let mut i = 0;
    loop {
        rprintln!("Hello from RTT! {}", i);
        writeln!(Uart, "HIGH").unwrap();
        gpio.set_high().unwrap();
        nb::block!(delay.wait()).unwrap();
        
        writeln!(Uart, "LOW").unwrap();
        gpio.set_low().unwrap();
        nb::block!(delay.wait()).unwrap();
        i +=1;
    }
}

// fn print_data()
