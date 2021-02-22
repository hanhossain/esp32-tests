#![no_std]

use esp32_hal::{clock_control::sleep, prelude::*};
use log::info;

pub fn countdown() {
    let mut counter = 10;
    loop {
        if counter > 1 {
            info!("Testing in {} seconds", counter);
        } else if counter == 1 {
            info!("Testing in {} second", counter);
        } else {
            break;
        }

        sleep(1.s());
        counter -= 1;
    }
}

pub fn complete() -> ! {
    info!("All tests passed!");

    loop {
        sleep(1.s());
    }
}
