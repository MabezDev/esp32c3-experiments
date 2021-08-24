#![no_std]


use void::Void;

use esp32c3::GPIO;

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
    
    pub fn new(index: u32) -> Self {

        let gpio = unsafe{ &*GPIO::ptr() };

        gpio.out_w1ts.modify(|_, w| unsafe { w.bits(0x1 << index) });
        gpio.func_out_sel_cfg[index as usize].modify(|_, w| unsafe { w.func0_out_sel().bits(0x80) });

        Self {
            index,
        }
    }
}

impl embedded_hal::digital::v2::OutputPin for GpioOutput {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let gpio = unsafe{ &*GPIO::ptr() };
        gpio.out_w1tc.modify(|_, w| unsafe { w.bits(0x1 << self.index) });
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let gpio = unsafe{ &*GPIO::ptr() };
        gpio.out_w1ts.modify(|_, w| unsafe { w.bits(0x1 << self.index) });
        Ok(())
    }
}

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

