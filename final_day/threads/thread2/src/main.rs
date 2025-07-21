#![no_std]
#![no_main]
//
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*, delay::Delay};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let gpiod = dp.GPIOD.split();

    // PD12 is usually connected to an onboard LED on STM32F429 Discovery
    let mut led = gpiod.pd12.into_push_pull_output();

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high();
        delay.delay_ms(500_u16);
        led.set_low();
        delay.delay_ms(500_u16);
    }
}