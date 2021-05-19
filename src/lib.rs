#![no_std]
#![feature(llvm_asm)]

use void::Void;

extern "C" {
    // ROM functions, see esp32c3-link.x
    pub fn uart_tx_one_char(byte: u8) -> i32;
    pub fn ets_delay_us(us: u32);
}

/// Naive gpio output implementation
pub struct GpioOutput {
    index: u32
}

impl GpioOutput {

    const GPIO_BASE: u32 = 0x60004000;

    /// GPIO output enable reg
    const GPIO_ENABLE_W1TS_REG: u32 = Self::GPIO_BASE + 0x0020;

    /// GPIO output set register
    const GPIO_OUT_W1TS_REG: u32 = Self::GPIO_BASE + 0x0008;
    /// GPIO output clear register
    const GPIO_OUT_W1TC_REG: u32 = Self::GPIO_BASE + 0x000C;


    /// GPIO function mode
    const GPIO_FUNCX_OUT_BASE: u32 = Self::GPIO_BASE + 0x0554;
    
    pub fn new(gpio: u32) -> Self {
        let funcx_sel: u32 = Self::GPIO_FUNCX_OUT_BASE + (gpio * 4);
        // configure the pin as an output
        unsafe {
            core::ptr::write_volatile(Self::GPIO_ENABLE_W1TS_REG as *mut _, 0x1 << gpio);
            // 0x100 makes this pin a simple gpio pin - see the technical reference for more info
            core::ptr::write_volatile(funcx_sel as *mut _, 0x80);
        }

        Self {
            index: gpio
        }
    }
}

impl embedded_hal::digital::v2::OutputPin for GpioOutput {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { core::ptr::write_volatile(Self::GPIO_OUT_W1TC_REG as *mut _, 0x1 << self.index) };
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { core::ptr::write_volatile(Self::GPIO_OUT_W1TS_REG as *mut _, 0x1 << self.index) };
        Ok(())
    }
}

/// uS timer
pub struct EtsTimer{
    delay: u32
}

impl EtsTimer {
    pub fn new(delay_us: u32) -> Self {
        Self {
            delay: delay_us,
        }
    }
}

impl embedded_hal::timer::CountDown for EtsTimer {
    type Time = u32;

    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time> {
        self.delay = count.into();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        Ok(unsafe { ets_delay_us(self.delay) })
    }
}

impl embedded_hal::timer::Periodic for EtsTimer {}

pub struct Uart;

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(for &b in s.as_bytes() {
            unsafe { uart_tx_one_char(b) };
        })
    }
}

pub fn disable_wdts() {
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
}

pub fn disable_interrupts() {
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe {
        let mut _tmp: u32;
        llvm_asm!("csrrs $0, mstatus, $1": "=r"(_tmp) : "rK"(0x00000008))
    };
}

pub fn enable_cycle_counter() {
    unsafe {
        llvm_asm!("csrw 0x7e0, $0" :: "rK"(0x01));
        llvm_asm!("csrw 0x7e1, $0" :: "rK"(0x01));
    }
}

pub fn get_cycle_count() -> u32 {
    let mut count: u32;
    unsafe {
        llvm_asm!("csrr $0, 0x07e2": "=r"(count) : "rK"(0x00000008))
    };
    count
}

