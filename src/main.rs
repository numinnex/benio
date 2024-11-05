use std::os::fd::RawFd;

use io_uring::{opcode, IoUring};
pub mod engine;

const QUEUE_DEPTH: u32 = 16;

fn main() {
    let mut ring = IoUring::new(QUEUE_DEPTH);
    // Basic test
    // Open a file (O_DIRECT | O_SYNC)
    // Do a stress write test of certain block sizes.
}

pub struct BenioFile {
    fd: RawFd,
}

impl BenioFile {
    pub fn new() {

    }
}
