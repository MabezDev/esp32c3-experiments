#![no_std]
#![no_main]

#![feature(llvm_asm)]

use panic_halt as _;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut _tmp: u32;
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe { llvm_asm!("csrrs $0, mstatus, $1": "=r"(_tmp) : "rK"(0x00000008)) };

    loop { continue; }
}