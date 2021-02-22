#![no_std]

use core::panic::PanicInfo;
use esp32_hal::{clock_control::sleep, prelude::*};
use log::{error, info};

pub fn setup() {
    esp32_logger::init();

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

    info!("Testing...");
}

pub fn complete() -> ! {
    info!("All tests passed!");

    loop {
        sleep(1.s());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    loop {}
}
