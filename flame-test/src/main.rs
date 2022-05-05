//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;
use imxrt_hal::{
    gpio::{Input, GPIO},
    iomuxc::{configure, gpio::Pin, Config, Hysteresis, PullKeeper},
};
use core::time::Duration;

mod logging;

const LED_PERIOD: Duration = Duration::from_millis(1_000);
/// The GPT output compare register we're using for
/// tracking time. This is the first register, since
/// we're using reset mode.
const GPT_OCR: bsp::hal::gpt::OutputCompareRegister = bsp::hal::gpt::OutputCompareRegister::One;

fn configure_switch<P: Pin>(mut switch_pin: P) -> GPIO<P, Input> {
    const LOW_SWITCH_CONFIG: Config = Config::zero()
        .set_hysteresis(Hysteresis::Enabled)
        .set_pull_keeper(Some(PullKeeper::Pulldown100k));
    configure(&mut switch_pin, LOW_SWITCH_CONFIG);
    GPIO::new(switch_pin)
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();

    // Reduce the number of pins to those specific
    // to the Teensy 4.0.
    let pins = bsp::pins::t40::from_pads(periphs.iomuxc);
    // Prepare the LED, and turn it on!
    // (If it never turns off, something
    // bad happened.)
    let mut led = bsp::configure_led(pins.p13);
    let flame_sensor = configure_switch(pins.p14);
    led.set();

    // Prepare the ARM clock to run at ARM_HZ.
    periphs.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut periphs.ccm.handle,
        &mut periphs.dcdc,
    );

    // Prepare a GPT timer for blocking delays.
    let mut timer = {
        // Run PERCLK on the crystal oscillator (24MHz).
        let mut cfg = periphs.ccm.perclk.configure(
            &mut periphs.ccm.handle,
            bsp::hal::ccm::perclk::PODF::DIVIDE_1,
            bsp::hal::ccm::perclk::CLKSEL::OSC,
        );

        let mut gpt1 = periphs.gpt1.clock(&mut cfg);
        // Keep ticking if we enter wait mode.
        gpt1.set_wait_mode_enable(true);
        // When the first output compare register compares,
        // reset the counter back to zero.
        gpt1.set_mode(bsp::hal::gpt::Mode::Reset);

        // Compare every LED_PERIOD_US ticks.
        gpt1.set_output_compare_duration(GPT_OCR, LED_PERIOD);
        gpt1
    };

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());

    timer.set_enable(true);
    loop {
        if flame_sensor.is_set() {
        led.toggle();
        }
        while !timer.output_compare_status(GPT_OCR).is_set() {}
        timer.output_compare_status(GPT_OCR).clear();
    }
}
