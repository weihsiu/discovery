#![no_main]
#![no_std]

use aux9::{entry, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    tim6.arr.write(|w| w.arr().bits(ms));
    tim6.cr1.write(|w| w.cen().enabled());
    while !tim6.sr.read().uif().bit_is_set() {}
    tim6.sr.write(|w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();

    // TODO initialize TIM6
    rcc.apb1enr.write(|w| w.tim6en().enabled());
    tim6.cr1.write(|w| w.opm().one_pulse().cen().disabled());
    tim6.psc.write(|w| w.psc().bits(7999));
    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
