#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};
// Panic handler that runs if a panic occurs in code.
// It provides an implementation that prints the address of a backtrace.
// These addresses can get decoded into source code locations.
use esp_backtrace as _;
// Provides `println!` implementation
use esp_println::println;

#[entry]
fn main() -> ! {
    // HAL drivers usually take ownership of peripherals accessed via the PAC
    let peripherals = Peripherals::take();
    // Sometimes a peripheral (here the System peripheral) is coarse-grained and
    // it does not exactly fit the HAL drivers.
    // So we split the System peripheral into smaller pieces which get passed to
    // the drivers.
    let system = peripherals.SYSTEM.split();

    // Configure the system clocks
    // In this case, defaults configuration is fine.
    // We freeze the clocks, which means we cannot change them later
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    // Create a delay starting from the frozen clocks
    let mut delay = Delay::new(&clocks);

    // Setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    // Log an information
    log::info!("Logger is setup");

    // Set GPIO8, the onboard LED, as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio8.into_push_pull_output();

    // Set GPIO8 pin state high initially
    led.set_high().unwrap();

    loop {
        // Toggle the GPIO8 pin state
        led.toggle().unwrap();
        // Delay a certain amount of milliseconds to create the blink effects
        delay.delay_ms(500u32);
        // Log an information about bliking
        println!("Blinking...");
    }
}
