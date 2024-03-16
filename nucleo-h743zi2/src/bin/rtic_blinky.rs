#![no_main]
#![no_std]

extern crate stm32h7xx_hal as stm32_hal;

use defmt_rtt as _; // global logger
use panic_probe as _;

#[rtic::app(device = stm32_hal::stm32, peripherals = true)]
mod app {
    //use cortex_m::Peripherals;
    use defmt::info;
    use stm32_hal::{block, pac::TIM1, prelude::*, timer::Timer};

    // Late resources
    #[shared]
    struct Shared {
        ld1: stm32_hal::gpio::gpiob::PB0<stm32_hal::gpio::Output<stm32_hal::gpio::PushPull>>,
        delay: stm32_hal::delay::Delay,
       
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        info!("init");

        info!("Starting blinky");

        // Get access to the device specific peripherals from the peripheral access crate
        let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

        // Get access to the core specific peripherals from the peripheral access crate
        let cp = stm32h7xx_hal::stm32::CorePeripherals::take().unwrap();

        // Take ownership over the RCC devices and convert them into the corresponding HAL structs
        let rcc = dp.RCC.constrain();

        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        // Freeze the configuration of all the clocks in the system and
        // retrieve the Core Clock Distribution and Reset (CCDR) object
        let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

        // Acquire the GPIOC peripheral
        let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

        // Configure gpio B pin 0 as a push-pull output.
        let mut ld1 = gpiob.pb0.into_push_pull_output();

        // Configure the timer to trigger an update every second
        //let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
        //timer.start(1.Hz());

        // Declare the delay conventional
        //let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let delay = stm32_hal::delay::Delay::new(cp.SYST, ccdr.clocks);

        ld1.set_low();

        (Shared {ld1, delay}, Local {}, init::Monotonics())
    }

    #[idle(shared = [ld1, delay])]
    fn idle(_ctx: idle::Context) -> ! {
        info!("idle");
        let mut delay = _ctx.shared.delay;
        let mut ld1 = _ctx.shared.ld1;

        let ms:u16 = 500;

        loop {
            info!("Blink Task");
            ld1.lock(|ld1|{ld1.toggle();});
            delay.lock(|delay|delay.delay_ms(ms));

        }
    }
}
