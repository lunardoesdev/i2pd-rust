use i2pd_sys;
use std::os::raw;

pub fn init(
    argc: raw::c_int,
    argv: *mut *mut raw::c_char,
    app_name: *const raw::c_char,
) {
    unsafe {
        i2pd_sys::C_InitI2P(argc, argv, app_name);
    }
}

pub fn terminate() {
    unsafe {
        i2pd_sys::C_TerminateI2P();
    }
}

pub fn start() {
    unsafe {
        i2pd_sys::C_StartI2P();
    }
}

pub fn stop() {
    unsafe {
        i2pd_sys::C_StopI2P();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start();
    }
}