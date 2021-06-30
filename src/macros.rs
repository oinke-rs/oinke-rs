#![allow(unused_macros)]

macro_rules! register_u16_rw {
    ($reg_name:ident, $reg_address:literal) => {
        #[repr(C)]
        pub struct $reg_name {
            r: RW<u16>,
        }

        impl $reg_name {
            pub fn get() -> &'static mut $reg_name {
                unsafe { &mut *($reg_address as *mut $reg_name) }
            }

            pub fn read(&mut self) -> u16 {
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                let r = self.r.read();
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                r
            }

            pub fn write(&mut self, bb: u16) {
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                unsafe { self.r.write(bb) };
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            }
        }
    };
}

macro_rules! busy_wait {
    ($nb_expr:expr, $exit_cond:expr) => {{
        loop {
            let res = $nb_expr;
            if res != Err(WouldBlock) {
                break res;
            }
            if $exit_cond {
                break res;
            }
        }
    }};
    ($nb_expr:expr) => {{
        loop {
            let res = $nb_expr;
            if res != Err(WouldBlock) {
                break res;
            }
        }
    }};
}

macro_rules! busy_wait_cycles {
    ($nb_expr:expr, $cycles:expr) => {{
        let started = crate::DWT::get_cycle_count();
        let cycles = $cycles;
        busy_wait!($nb_expr, crate::DWT::get_cycle_count().wrapping_sub(started) >= cycles)
    }};
    ($cycles:expr) => {{
        let started = crate::DWT::get_cycle_count();
        let cycles = $cycles;
        loop {
            if crate::DWT::get_cycle_count().wrapping_sub(started) >= cycles {
                break;
            }
        }
    }};
}

macro_rules! bench_it {
    ($myexpr:expr) => {{
        let started = crate::DWT::get_cycle_count();
        let r = $myexpr;
        (r, crate::DWT::get_cycle_count().wrapping_sub(started))
    }};
}

#[cfg(feature = "logs")]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut output = jlink_rtt::NonBlockingOutput::new();
        let _ = writeln!(&mut output, $($arg)*);
        //if let Some(term) = unsafe { &mut crate::TERM } {
        //    let _ = writeln!(term, $($arg)*);
        //}
    });
}

#[cfg(feature = "logs")]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut output = jlink_rtt::NonBlockingOutput::new();
        let _ = write!(&mut output, $($arg)*);
    });
}

#[cfg(not(feature = "logs"))]
macro_rules! println {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(feature = "logs"))]
macro_rules! print {
    ($($arg:tt)*) => {{}};
}
