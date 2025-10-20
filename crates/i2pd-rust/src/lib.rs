pub use i2pd_sys;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            i2pd_sys::C_StartI2P(); 
        }
    }
}
