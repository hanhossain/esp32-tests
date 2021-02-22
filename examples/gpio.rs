#![no_std]
#![no_main]

use esp32_hal::{
    clock_control::{ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    prelude::*,
    target,
    timer::Timer,
};
use esp32_tests as tests;
use log::info;

// Assumptions:
// - GPIO 34 is tied to 3.3V
// - GPIO 17 is tied to GND
#[entry]
fn main() -> ! {
    let dp = target::Peripherals::take().expect("failed to acquire peripherals");
    let (_, dport_clock_control) = dp.DPORT.split();

    let clock_control = ClockControl::new(
        dp.RTCCNTL,
        dp.APB_CTRL,
        dport_clock_control,
        XTAL_FREQUENCY_AUTO,
    )
    .unwrap();

    // disable RTC watchdog
    let (clock_control_config, mut watchdog) = clock_control.freeze().unwrap();
    watchdog.disable();

    // disable MST watchdogs
    let (.., mut watchdog0) = Timer::new(dp.TIMG0, clock_control_config);
    let (.., mut watchdog1) = Timer::new(dp.TIMG1, clock_control_config);
    watchdog0.disable();
    watchdog1.disable();

    let gpios = dp.GPIO.split();

    tests::setup();

    info!("Test: Floating input is high");
    let input_3_3v = gpios.gpio34.into_floating_input();
    assert!(input_3_3v.is_high().unwrap());
    info!("Passed!");

    info!("Test: Floating input is low");
    let input_0v = gpios.gpio17.into_floating_input();
    assert!(input_0v.is_low().unwrap());
    info!("Passed!");

    tests::complete();
}
