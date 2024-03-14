// We are going to use a finite state machine to turn on a LED0, Turn off LED0 and Turn on LED1 and Turn off LED1 and Turn on LED2 e and again
// LED0 -> PB0
// LED1 -> PE1
// LED2 -> PB14
// PB0 -> PC13

#![no_std]
#![no_main]

use defmt_rtt as _; // global logger

use defmt::info;
use panic_probe as _;

use stm32h7xx_hal::{block, prelude::*, timer::Timer};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    info!("Start FSM");

    // Get the acess to the specific device peripherals from peripheral acess crate 
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Take the ownership of Reset and Clock Control (RCC) devices and convert them into the corresponding HAL Structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze in the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) objects
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);


    // Acquire the GPIOB peripheral 
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Acquire the GPIOC peripheral 
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    // Acquire the GPIOE peripheral 
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Configure the GPIO B pin 0 into output LED0
    let mut ld0 = gpiob.pb0.into_push_pull_output();

    // Configure the GPIO E pin 1 into output LED1
    let mut ld1 = gpioe.pe1.into_push_pull_output();

    // Configure the GPIO B pin 14 into output LED2
    let mut ld2 = gpiob.pb14.into_push_pull_output();

    // Configure the GPIO C pin 13 into Input PB1
    let pb1 = gpioc.pc13.into_input();


    // Configure the timer to trigger an update every second
    let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.Hz());


    info!("Entering in the main loop");
    loop {
        ld0.set_high();
        block!(timer.wait()).unwrap();
        ld0.set_low();
        block!(timer.wait()).unwrap();
    }

}

