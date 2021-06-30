use core::panic::PanicInfo;
use core::sync::atomic::*;

use cortex_m::interrupt;
use cortex_m_rt::{exception, ExceptionFrame};

pub fn _reset() -> ! {
    cortex_m::interrupt::disable();
    cortex_m::peripheral::SCB::sys_reset()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();
    log::error!("Panic handler! Reseting...");
    log::error!("Panic info : {}", info);
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    interrupt::disable();
    log::error!("HardFault handler!");
    log::error!("{:?}", &ef);
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
