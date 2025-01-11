#![deny(unsafe_code)]
#![no_main]
#![no_std]

use defmt::info;
// Print panic message to probe console
use panic_probe as _;

use cortex_m_rt::entry;
use defmt_rtt as _;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8u32.MHz()).freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    loop {
        led.set_high().unwrap();
        info!("High");
        Delay::delay_ms(&mut delay, 1000u32);
        led.set_low().unwrap();
        info!("Low");
        Delay::delay_ms(&mut delay, 1000u32);
    }
}
