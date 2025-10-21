use i2pd_sys;
use std::os::raw;
use std::ffi::CString;

pub fn init_raw(
    argc: raw::c_int,
    argv: *mut *mut raw::c_char,
    app_name: *const raw::c_char,
) {
    unsafe {
        i2pd_sys::C_InitI2P(argc, argv, app_name);
    }
}

pub fn init(
    argv: &str,
    app_name: &str,
) {
    let cstrings: Vec<CString> = argv
        .split(" ")
        .map(|s| CString::new(s)
            .expect("failed to create CString"))
        .collect();
    
    let mut cargv: Vec<*mut raw::c_char> = cstrings
        .iter()
        .map(|cs| cs.as_ptr() as *mut raw::c_char)
        .collect();

    let cappname = CString::new(app_name)
        .expect("failed to create CString for appname");
    
    let argc: raw::c_int = cargv.len() as raw::c_int;
    
    init_raw(argc, cargv.as_mut_ptr(), cappname.as_ptr());
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
        init("--version", "appname");
        start();
    }
}