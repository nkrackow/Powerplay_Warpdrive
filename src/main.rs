#![no_std]
#![cfg_attr(not(doc), no_main)]

#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    use panic_rtt_target as _;
    use rtt_target::{rprintln, rtt_init_print};
    use cortex_m_rt::entry;
    use nb::block;
    use stm32f1xx_hal::{
        gpio::{gpioa::PA0, gpioc::PC13, Edge, ExtiPin, Input, Output, PullDown, PushPull},
        prelude::*,
    };
    use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("Hello, Rust!");

        let mut flash = ctx.device.FLASH.constrain();
        let rcc = ctx.device.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut timer = Timer::syst(ctx.core.SYST, &clocks).counter_hz();
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
        (Shared {}, Local {}, init::Monotonics())
    }
}
