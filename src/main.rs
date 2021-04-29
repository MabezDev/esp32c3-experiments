#![no_std]
#![no_main]

#![feature(llvm_asm)]

use panic_halt as _;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe { 
        let mut _tmp: u32;
        llvm_asm!("csrrs $0, mstatus, $1": "=r"(_tmp) : "rK"(0x00000008)) 
    };

    // disable wdt's
    unsafe {
        // tg0 wdg
        core::ptr::write_volatile(0x6001f064 as *mut _, 0x50D83AA1); // disable write protect
        core::ptr::write_volatile(0x6001F048 as *mut _, 0);
        core::ptr::write_volatile(0x6001f064 as *mut _, 0); // enable write protect

        // tg1 wdg
        core::ptr::write_volatile(0x60020064 as *mut _, 0x50D83AA1); // disable write protect
        core::ptr::write_volatile(0x60020048 as *mut _, 0);
        core::ptr::write_volatile(0x60020064 as *mut _, 0); // enable write protect

        // rtc wdg
        core::ptr::write_volatile(0x600080a8 as *mut _, 0x50D83AA1); // disable write protect
        core::ptr::write_volatile(0x60008090 as *mut _, 0);
        core::ptr::write_volatile(0x600080a8 as *mut _, 0); // enable write protect
    }

    loop {}
}