#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::*;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let period = 50u16;
    let mut count = 0;
    loop {
        for i in 0..8 {
            let j = (i + 1) % 8;
            leds[j].on();
            delay.delay_ms(period);
            leds[i].off();
            delay.delay_ms(period);
        }
    }
}
