#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Ticker, Timer};
use hal::gpio::{DriveStrength, Level, Output, SlewRate};
use {defmt_rtt as _, embassy_mcxa as hal, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(hal::config::Config::default());

    defmt::info!("Blink example");

    let mut red = Output::new(p.P3_18, Level::High, DriveStrength::Normal, SlewRate::Fast);
    let mut green = Output::new(p.P3_19, Level::High, DriveStrength::Normal, SlewRate::Fast);
    let mut blue = Output::new(p.P3_21, Level::High, DriveStrength::Normal, SlewRate::Fast);

    let mut ticker = Ticker::every(Duration::from_millis(250));

    loop {
        for _ in 0..4 {
            ticker.next().await;
        }

        let now = Instant::now();
        while now.elapsed() < Duration::from_millis(100) {}
        red.toggle();

        // red.toggle();
        // ticker.next().await;

        // red.toggle();
        // green.toggle();
        // ticker.next().await;

        // green.toggle();
        // blue.toggle();
        // ticker.next().await;
        // blue.toggle();

        // ticker.next().await;
        // ticker.next().await;
    }
}
