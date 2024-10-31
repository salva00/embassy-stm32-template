#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {


    {% if chip contains "stm32" -%}
    let p = embassy_stm32::init(Default::default());
    {% endif -%}

    {% if chip contains "stm32f4" -%}
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);
    {% endif -%}


    loop {
        defmt::info!("Blink");
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
