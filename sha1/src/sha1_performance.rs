#![allow(unused_variables)]

// Will also be used in the storage-efficient implementation

// TODO: Remove unsafe, implement message to word array, and make the code ergonomic

use std::io::Read;
use std::path::PathBuf;

macro_rules! shift_rotate {
    ($num:literal ,$expression:expr) => {
        ($expression).rotate_left($num)
    };
}

pub struct Sha1 {
    num_blocks: u64,
    message_length: u64,
    message_block_index: u64,
    f_buf: [u32; 5],
    h_buf: [u32; 5],
    word: [u32; 80],
}

impl Sha1 {
    pub fn new() -> Self {
        Self {
            num_blocks: 0,
            message_length: 0,
            message_block_index: 0,
            f_buf: [0; 5],
            h_buf: [0; 5],
            word: [0; 80],
        }
    }

    pub fn initiate_file(&mut self, message: PathBuf) {
        let mut file = std::fs::File::open(message).unwrap();
        let mut message = Vec::new();
        file.read_to_end(&mut message).unwrap();
        let len = message.len();
    }

    fn compute_hash(&mut self) {
        // for t in 0..16 {
        //     self.word[t] = message[t];
        // }
        for t in 16..80 {
            self.word[t] = shift_rotate!(
                1,
                self.word[t - 3] ^ self.word[t - 8] ^ self.word[t - 14] ^ self.word[t - 16]
            );
        }
        for t in 0..5 {
            self.f_buf[t] = self.h_buf[t].clone();
        }
        for t in 0..80 {
            let temp =
                self.f_buf[0].rotate_left(1) /* + f(x, y, z) */ + self.f_buf[4] + self.word[t];
            self.f_buf[4] = self.f_buf[3];
            self.f_buf[3] = self.f_buf[2];
            self.f_buf[2] = self.f_buf[1].rotate_left(30);
            self.f_buf[1] = self.f_buf[0];
            self.f_buf[0] = temp;

            // Unimplemented: constants (K);
        }
        for t in 0..5 {
            self.h_buf[t] = self.h_buf[t] + self.f_buf[t];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {}
}
