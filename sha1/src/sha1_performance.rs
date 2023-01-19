#![allow(unused_variables)]

// Will also be used in the storage-efficient implementation

// TODO: Remove unsafe, implement message to word array, and make the code ergonomic

use crate::constants::K;
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
            h_buf: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0],
            word: [0; 80],
        }
    }
    pub fn get_hash(&self) -> [u32; 5] {
        self.h_buf
    }
    pub fn get_words(&self, n: usize) -> u32 {
        self.word[n]
    }
    pub fn process_hash(&mut self, input: &[u8]) {
        let temp: &mut [u8; 64] = &mut [0; 64];
        let length = input.len() as u64;
        self.message_length = length * 8;
        temp.iter_mut().zip(input).for_each(|(t, i)| {
            *t = *i;
        });
        temp[input.len()] = temp[input.len()] | 0x80;
        // for i in 56..64 {
        //     temp[i] = ((length * 8) << ((i - 56) * 8)) as u8;
        // }
        self.hash(temp);
    }
    fn hash(&mut self, blocks: &[u8; 64]) {
        self.word
            .iter_mut()
            .zip(0..16)
            .enumerate()
            .for_each(|(tlen, (word, _))| {
                let len = tlen * 4;
                *word = (blocks[len + 3] as u32)
                    | (blocks[len + 2] as u32) << 8
                    | (blocks[len + 1] as u32) << 16
                    | (blocks[len] as u32) << 24;
            });
        self.word[14] = (self.message_length & 0xff00) as u32;
        self.word[15] = (self.message_length & 0x00ff) as u32;
        self.compute_hash();
    }

    // pub fn initiate_file(&mut self, message: PathBuf) {
    //     let mut file = std::fs::File::open(message).unwrap();
    //     let mut message = Vec::new();
    //     file.read_to_end(&mut message).unwrap();
    //     let len = message.len();
    // }
    // pub fn from_str(msg: &str) {
    //     // let bytes = msg.bytes().collect();
    // }
    fn f(&self, i: &usize) -> u32 {
        if *i < 20 {
            (self.f_buf[1] & self.f_buf[2]) | (!self.f_buf[1] & self.f_buf[3])
        } else if *i >= 40 || *i <= 59 {
            (self.f_buf[1] & self.f_buf[2])
                | (self.f_buf[1] & self.f_buf[3])
                | (self.f_buf[2] & self.f_buf[3])
        } else {
            self.f_buf[1] ^ self.f_buf[2] ^ self.f_buf[3]
        }
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
            self.f_buf[t] = self.h_buf[t];
        }
        for t in 0..80_usize {
            let mut temp: u32 = 0;
            let idx = ((t) / 20) as usize;
            temp = temp
                .wrapping_add(self.f_buf[0].rotate_left(5))
                .wrapping_add(self.f(&t))
                .wrapping_add(self.f_buf[4])
                .wrapping_add(self.word[t])
                .wrapping_add(K[idx]);
            self.f_buf[4] = self.f_buf[3];
            self.f_buf[3] = self.f_buf[2];
            self.f_buf[2] = self.f_buf[1].rotate_left(30);
            self.f_buf[1] = self.f_buf[0];
            self.f_buf[0] = temp;
        }
        for t in 0..5 {
            self.h_buf[t] = self.h_buf[t].wrapping_add(self.f_buf[t]);
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn hash() {
        let mut sha = Sha1::new();
        sha.process_hash(b"abc");
        assert_eq!(
            sha.get_hash(),
            [0xA9993E36, 0x4706816A, 0xBA3E2571, 0x7850C26C, 0x9CD0D89D]
        );
    }
    #[test]
    fn first_word() {
        let mut sha = Sha1::new();
        sha.process_hash(b"abc");
        assert_eq!(sha.get_words(0), 0x61626380);
    }
    #[test]
    fn second_word() {
        let mut sha = Sha1::new();
        sha.process_hash(b"abc");
        assert_eq!(sha.get_words(1), 0x0);
    }
    #[test]
    fn last_word() {
        let mut sha = Sha1::new();
        sha.process_hash(b"abc");
        assert_eq!(sha.get_words(15), 0x18);
    }
    #[test]
    fn seven_word() {
        let mut sha = Sha1::new();
        sha.process_hash(b"abc");
        assert_eq!(sha.get_words(7), 0x0);
    }
}
