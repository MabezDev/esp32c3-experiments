#![no_std]
#![no_main]
#![feature(llvm_asm)]

use panic_halt as _;

use riscv_rt::entry;

const GPIO_BASE: u32 = 0x60004000;

/// GPIO output enable reg
const GPIO_ENABLE_W1TS_REG: u32 = GPIO_BASE + 0x0020;

/// GPIO output set register
const GPIO_OUT_W1TS_REG: u32 = GPIO_BASE + 0x0008;
/// GPIO output clear register
const GPIO_OUT_W1TC_REG: u32 = GPIO_BASE + 0x000C;

const BLINKY_GPIO: u32 = 18;

/// GPIO function mode
const GPIO_FUNCX_OUT_BASE: u32 = GPIO_BASE + 0x0554;
const GPIO_FUNCX_OUT_SEL_CFG: u32 = GPIO_FUNCX_OUT_BASE + (BLINKY_GPIO * 4);

#[entry]
fn main() -> ! {
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe {
        let mut _tmp: u32;
        llvm_asm!("csrrs $0, mstatus, $1": "=r"(_tmp) : "rK"(0x00000008))
    };

    // disable wdt's
    unsafe {
        // super wdt
        core::ptr::write_volatile(0x600080B0 as *mut _, 0x8F1D312Au32); // disable write protect
        core::ptr::write_volatile(
            0x600080AC as *mut _,
            core::ptr::read_volatile(0x600080AC as *const u32) | 1 << 31,
        ); // set RTC_CNTL_SWD_AUTO_FEED_EN
        core::ptr::write_volatile(0x600080B0 as *mut _, 0u32); // enable write protect

        // tg0 wdg
        core::ptr::write_volatile(0x6001f064 as *mut _, 0x50D83AA1u32); // disable write protect
        core::ptr::write_volatile(0x6001F048 as *mut _, 0u32);
        core::ptr::write_volatile(0x6001f064 as *mut _, 0u32); // enable write protect

        // tg1 wdg
        core::ptr::write_volatile(0x60020064 as *mut _, 0x50D83AA1u32); // disable write protect
        core::ptr::write_volatile(0x60020048 as *mut _, 0u32);
        core::ptr::write_volatile(0x60020064 as *mut _, 0u32); // enable write protect

        // rtc wdg
        core::ptr::write_volatile(0x600080a8 as *mut _, 0x50D83AA1u32); // disable write protect
        core::ptr::write_volatile(0x60008090 as *mut _, 0u32);
        core::ptr::write_volatile(0x600080a8 as *mut _, 0u32); // enable write protect
    }

    // configure the pin as an output
    unsafe {
        core::ptr::write_volatile(GPIO_ENABLE_W1TS_REG as *mut _, 0x1 << BLINKY_GPIO);
        // 0x100 makes this pin a simple gpio pin - see the technical reference for more info
        core::ptr::write_volatile(GPIO_FUNCX_OUT_SEL_CFG as *mut _, 0x80);
    }

    loop {
        unsafe {
            // turn on the LED
            core::ptr::write_volatile(GPIO_OUT_W1TS_REG as *mut _, 0x1 << BLINKY_GPIO);
            ets_delay_us(1_000_000);
            // turn off the LED`
            core::ptr::write_volatile(GPIO_OUT_W1TC_REG as *mut _, 0x1 << BLINKY_GPIO);
            ets_delay_us(1_000_000);
        }
    }
}

extern "C" {
    // ROM functions, see esp32c3-link.x
    fn ets_delay_us(us: u32);
}
