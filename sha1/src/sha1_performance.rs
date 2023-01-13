#![allow(unused_variables)]

pub struct Sha1<'a> {
    num_blocks: u64,
    message_length: u64,
    message_block_index: u64,
    message: &'a [u8],
}

impl<'a> Sha1<'a> {
    fn new(message: &[u8]) {
        let len = message.len();
        unimplemented!()
    }
}
