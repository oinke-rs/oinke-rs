#![no_std]
#![no_main]
#![allow(clippy::identity_op)]

use stm32f4xx_hal::pac::DWT;
use stm32f4xx_hal::{i2c::I2c, prelude::*};


#[macro_use]
mod macros;
#[cfg(not(feature = "prod"))]
mod emblog;
mod sys;


#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        a: i32,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        emblog::init().unwrap();
        log::info!("OINKE-RS startup!");

        // INIT HW
        {
            let _cp = cx.core;
            let dp = cx.device;
            let rcc = dp.RCC.constrain();

            let clocks = rcc
                .cfgr
                .use_hse(25.mhz())
                .sysclk(72.mhz())
                .freeze();
            
            let gpiob = dp.GPIOB.split();
            let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
            let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
            let i2c = I2c::new(dp.I2C1, (scl, sda), 100.khz(), clocks);
            
            // Hardcoded I2C address 0x76;
            let mut ps = bmp280_ehal::BMP280::new(i2c).unwrap();
            // to get pressure:
            let pres = ps.pressure();
            println!("{:?}", pres);
        }

        init::LateResources { a: 0 }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            continue;
        }
    }

};
