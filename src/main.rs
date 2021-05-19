#![no_std]
#![no_main]

use embedded_hal::prelude::*;
use panic_halt as _;

use core::fmt::Write;
use riscv_rt::{TrapFrame, entry};

use esp32c3_lib::{EtsTimer, GpioOutput, Uart, disable_wdts, enable_cycle_counter, get_cycle_count};

use smart_leds::{SmartLedsWrite, RGB8};

#[entry]
fn main() -> ! {
    

    // disable wdt's
    disable_wdts();

    writeln!(Uart, "Hello world!").unwrap();

    enable_cycle_counter();

    writeln!(Uart, "MCYCLE A: {}", get_cycle_count()).unwrap();
    
    let mut delay = EtsTimer::new(1_000_000);
    
    let mut data: [RGB8; 3] = [RGB8::default(); 3];
    let empty: [RGB8; 3] = [RGB8::default(); 3];
    let mut ws = ws2812_timer_delay::Ws2812::new(EtsTimer::new(1), GpioOutput::new(8));
    
    writeln!(Uart, "MCYCLE B: {}", get_cycle_count()).unwrap();
    
    loop {
        data[0] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[1] = RGB8 {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[2] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };
        ws.write(data.iter().cloned()).unwrap();
        nb::block!(delay.wait()).unwrap();
        ws.write(empty.iter().cloned()).unwrap();
        nb::block!(delay.wait()).unwrap();
    }
}

#[export_name = "ExceptionHandler"]
fn esp32c3_exception(_trap_frame: &TrapFrame) -> ! {
    panic!("EXCEPTION")
}
