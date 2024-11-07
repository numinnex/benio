use std::os::fd::RawFd;

pub mod engine;

const QUEUE_DEPTH: u32 = 16;

fn main() {
    // Basic test
    // Open a file (O_DIRECT | O_SYNC)
    // Do a stress write test of certain block sizes.
}

pub struct BenioFile {
    fd: RawFd,
}

impl BenioFile {
    pub fn raw_fd(&self) -> RawFd {
        self.fd
    }
}
