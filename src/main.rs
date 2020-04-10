#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt;
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f4;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    // hprintln!("hello world!").unwrap();

    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpiog = dp.GPIOG.split();
        let mut led = gpiog.pg13.into_push_pull_output();
        
        for i in 1.. {
            hprintln!("i: {}", i).unwrap();
            led.toggle().unwrap();
        }
    }
    
    loop {
        asm::wfe();
    }
}
