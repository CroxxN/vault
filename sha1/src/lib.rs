#![allow(dead_code)]
#![allow(unused_imports)]

mod constants;
mod error;
mod sha1_performance;

use std::path;
#[derive(Debug, PartialEq)]
enum CalcMode {
    Performance,
    Storage,
}

pub struct Sha1Builder {
    calc_mode: CalcMode,
}

pub struct Sha1 {
    mode: CalcMode,
}

impl Default for Sha1Builder {
    fn default() -> Self {
        Sha1Builder {
            calc_mode: (CalcMode::Performance),
        }
    }
}

impl Sha1Builder {
    pub fn performance(self) -> Self {
        Self {
            calc_mode: CalcMode::Performance,
        }
    }
    pub fn storage(self) -> Self {
        Self {
            calc_mode: CalcMode::Storage,
        }
    }
    pub fn build(self) -> Sha1 {
        Sha1 {
            mode: self.calc_mode,
        }
    }
}

impl Sha1 {
    pub fn new() -> Sha1 {
        Sha1 {
            mode: CalcMode::Performance,
        }
    }
    pub fn new_opts() -> Sha1Builder {
        Sha1Builder::default()
    }
    fn new_from_file(&self) {
        unimplemented!()
    }
    fn from_bytes(_: &[u8]) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sha1 = Sha1::new();
        assert_eq!(sha1.mode, CalcMode::Performance);
    }
    fn new_with_opts() {
        let sha1 = Sha1::new_opts().storage().build();
        assert_eq!(sha1.mode, CalcMode::Storage);
    }
}
