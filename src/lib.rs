pub mod bindi2pd;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn starto() {
  unsafe {
    bindi2pd::C_StartI2P();
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_starts() {
      starto();
    }
}
