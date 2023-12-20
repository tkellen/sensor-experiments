#![no_std]
#![no_main]

use bsp::board;
use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::hal::timer::Blocking;

const DELAY_MS: u32 = 500;

// We're responsible for configuring our timers. This example uses PERCLK_CLK
// as the GPT1 clock source, and it configures a 1 KHz GPT1 frequency by
// computing a GPT1 divider.
use bsp::hal::gpt::ClockSource;
/// The intended GPT1 frequency (Hz).
const GPT1_FREQUENCY: u32 = 1_000;
/// Given this clock source...
const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
/// ... the root clock is PERCLK_CLK. To configure a GPT1 frequency,
/// we need a divider of...
const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;

#[bsp::rt::entry]
fn main() -> ! {
    let instances = board::instances();
    let board::Resources {
        pins,
        mut gpt1,
        usb,
        mut gpio2,
        ..
    } = board::t41(instances);
    bsp::LoggingFrontend::default_log().register_usb(usb);

    gpt1.disable();
    gpt1.set_divider(GPT1_DIVIDER);
    gpt1.set_clock_source(GPT1_CLOCK_SOURCE);

    let mut delay = Blocking::<_, GPT1_FREQUENCY>::from_gpt(gpt1);
    let led = board::led(&mut gpio2, pins.p13);

    let mut counter: u32 = 0;
    loop {
        led.toggle();
        log::info!("Hello from the USB logger! The count is {counter}");
        delay.block_ms(DELAY_MS);
        counter = counter.wrapping_add(1);
    }
}

