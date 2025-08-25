#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{
    clock::CpuClock,
    main,
    time::{Duration, Instant},
};

use esp_println::println;
// use log::info;

use esp_backtrace as _;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.4.0

    #[cfg(not(feature = "test"))]
    {
        // --- Normal production firmware branch ---
        esp_println::logger::init_logger_from_env();
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let _peripherals = esp_hal::init(config);

        // Production code goes here
        println!("Running production firmware...");

        loop {
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(500) {}
        }
    }

    #[cfg(feature = "test")]
    {
        // #region test specific usings start
        use rpn::esp_test;
        // #endregion test specific usings end
        // --- Test firmware branch ---
        esp_println::logger::init_logger_from_env();
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let _peripherals = esp_hal::init(config);

        // Call the test framework
        esp_test::entry::run();

        // Loop to prevent returning
        loop {
            println!("Idle...");
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(500) {}
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
