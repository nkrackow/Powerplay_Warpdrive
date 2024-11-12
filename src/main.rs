#![no_std]
#![cfg_attr(not(doc), no_main)]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use nb::block;

use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, Rust!");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();


    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    let mut i = 0;
    loop {
        block!(timer.wait()).unwrap();
        i += 1;
        rprintln!("Hello again; I have blinked {} times.", i);
        if i == 10 {
            panic!("Yow, 10 times is enough!");
        }
    }
}
