#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mcxa::clocks::{PoweredClock, config::{Div8, Fro16KConfig, MainClockConfig, MainClockSource}};
use embassy_time::{Duration, Instant, Ticker};
use hal::gpio::{DriveStrength, Level, Output, SlewRate};
use {defmt_rtt as _, embassy_mcxa as hal, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut cfg = hal::config::Config::default();
    cfg.clock_cfg.firc = None;
    cfg.clock_cfg.sirc.fro_12m_enabled = false;
    cfg.clock_cfg.sirc.fro_lf_div = None;
    let mut fro = Fro16KConfig::default();
    fro.vsys_domain_active = true;
    fro.vdd_core_domain_active = true;
    cfg.clock_cfg.fro16k = Some(fro);
    cfg.clock_cfg.sosc = None;
    cfg.clock_cfg.spll = None;
    cfg.clock_cfg.main_clock = MainClockConfig {
        source: MainClockSource::RoscFro16K,
        power: PoweredClock::NormalEnabledDeepSleepDisabled,
        ahb_clk_div: Div8::no_div(),
    };
    let p = hal::init(cfg);

    defmt::info!("Blink example");

    let mut red = Output::new(p.P3_18, Level::High, DriveStrength::Normal, SlewRate::Fast);
    // let mut green = Output::new(p.P3_19, Level::High, DriveStrength::Normal, SlewRate::Fast);
    // let mut blue = Output::new(p.P3_21, Level::High, DriveStrength::Normal, SlewRate::Fast);

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
