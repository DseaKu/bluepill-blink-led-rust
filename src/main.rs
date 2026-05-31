#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Acquire peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze clocks
    let mut rcc = rcc.freeze(stm32f1xx_hal::rcc::Config::default(), &mut flash.acr);

    // Acquire GPIOC port
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure PC13 as a push-pull output
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Create a delay abstraction using the system timer
    // The clocks are now accessed via the rcc struct.
    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        // PC13 is active-low on the Blue Pill
        led.set_low();
        delay.delay_ms(500_u16);

        led.set_high();
        delay.delay_ms(500_u16);
    }
}
