#![no_std]
#![no_main]
#![allow(clippy::identity_op)]

use stm32f4xx_hal::pac::DWT;
use stm32f4xx_hal::{i2c::I2c, prelude::*};
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use usb_device::prelude::*;
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::MouseReport;
use usbd_hid::hid_class::HIDClass;


#[macro_use]
mod macros;
#[cfg(not(feature = "prod"))]
mod emblog;
mod sys;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

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
                .require_pll48clk()
                .freeze();
            
            let gpioa = dp.GPIOA.split();
            let gpiob = dp.GPIOB.split();
            let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
            let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
            let _i2c = I2c::new(dp.I2C1, (scl, sda), 100.khz(), clocks);

            // Hardcoded I2C address 0x76;
            //let mut ps = bmp280_ehal::BMP280::new(i2c).unwrap();
            //println!("id: {:?}", ps.id());
            //println!("press oneshot: {}", ps.pressure_one_shot());
            

            let usb = USB {
                usb_global: dp.OTG_FS_GLOBAL,
                usb_device: dp.OTG_FS_DEVICE,
                usb_pwrclk: dp.OTG_FS_PWRCLK,
                pin_dm: gpioa.pa11.into_alternate_af10(),
                pin_dp: gpioa.pa12.into_alternate_af10(),
                hclk: clocks.hclk(),
            };

            let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });
            let mut mouse = HIDClass::new(&usb_bus, MouseReport::desc(), 60);
            //let mut keyboard = HIDClass::new(&usb_bus, Keyboard::desc(), 60);

            let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Twitchy Mousey")
                .serial_number("TEST")
                .device_class(0xEF) // misc
                .build();

            let mut twitch_toggle = false;
            let mut cnt = 0usize;
            loop {
                if usb_dev.poll(&mut [&mut mouse]) {
                    if cnt % 1 == 0 {
                        let _ = mouse.push_input(&
                            if twitch_toggle {
                                MouseReport {
                                    x: 0,
                                    y: 4,
                                    buttons: 0,
                                    wheel: 0,
                                }
                            } else {
                                MouseReport {
                                    x: 0,
                                    y: -4,
                                    buttons: 0,
                                    wheel: 0,
                                }
                            }
                        );
                        twitch_toggle ^= true;
                    }
                }
                cnt = cnt.wrapping_add(1);
            }

            // to get pressure:
            //loop {
            //    let pres = ps.pressure();
            //    println!("{:?}", pres);
            //    busy_wait_cycles!(72000 * 10);
            //}
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
