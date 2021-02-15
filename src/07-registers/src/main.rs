#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (mut itm, gpioe) = aux7::init();

    gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    iprintln!(&mut itm.stim[0], "done {}", gpioe.odr.read().odr9().bit());

    // unsafe {
    //     // A magic address!
    //     const GPIOE_BSRR: u32 = 0x48001018;

    //     // Turn on the "North" LED (red)
    //     *(GPIOE_BSRR as *mut u32) = 1 << 9;

    //     // Turn on the "East" LED (green)
    //     *(GPIOE_BSRR as *mut u32) = 1 << 11;

    //     // Turn off the "North" LED
    //     *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

    //     // Turn off the "East" LED
    //     *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    // }

    loop {}
}
