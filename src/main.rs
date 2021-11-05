#![no_std]
#![no_main]
#![feature(asm)]

use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_timer_CountDown};
use riscv_atomic_emulation_trap::atomic_emulation;

use core::fmt::Write;
use riscv_rt::entry;

use esp32c3_lib::{disable_wdts, EtsTimer, GpioOutput, Uart};

use core::sync::atomic::Ordering;

// make sure we have something in our data section
#[used]
static DATA_SECTION_TEST: &'static str = "TEST DATA";
// make sure we have something in our bss section
#[used]
static mut BSS_SECTION_TEST: [u8; 12] = [0xAA; 12];

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
    writeln!(Uart, "{}", DATA_SECTION_TEST).unwrap();

    let mut delay = EtsTimer::new(1_000_000);

    let a = core::sync::atomic::AtomicUsize::new(1);
    let x = a.compare_exchange(1, 2, Ordering::Acquire, core::sync::atomic::Ordering::Relaxed).unwrap();
    writeln!(Uart, "Value of x: {}", x).unwrap();

    loop {
        writeln!(Uart, "HIGH").unwrap();
        gpio.set_high().unwrap();
        nb::block!(delay.wait()).unwrap();

        writeln!(Uart, "LOW").unwrap();
        gpio.set_low().unwrap();
        nb::block!(delay.wait()).unwrap();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    writeln!(Uart, "Panic: {:#?}", info).ok();
    loop{}
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn ExceptionHandler(riscv: &mut riscv_atomic_emulation_trap::TrapFrame) {
    writeln!(Uart, "Handling exception at 0x{:08X}", riscv::register::mepc::read()).ok();
    writeln!(Uart, "Trap before: {:?}", riscv).ok();
    if atomic_emulation(riscv) {
        writeln!(Uart, "Trap after: {:?}", riscv).ok();
        // successfull emulation, move the mepc
        riscv::register::mepc::write(riscv::register::mepc::read() + core::mem::size_of::<usize>())
    } else {
        panic!("Unhandled exception at 0x{:08X}", riscv::register::mepc::read());
    }
}
