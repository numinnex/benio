use engine::Engine;
use std::os::fd::RawFd;

pub mod context;
pub mod engine;
pub mod io_unit;

const QUEUE_DEPTH: u32 = 16;

fn main() {
    let mut engine = engine::io_uring::IoUring::init(QUEUE_DEPTH);
    let file = engine.open_file("test").expect("Failed to create file");

    engine.close_file(file).expect("Failed to close file");
}

pub struct BenioFile {
    fd: RawFd,

    //TODO: use this for registered buffers.
    engine_position: u64,
    //TODO add state required for randomly sampled I/O
}

impl BenioFile {
    pub fn raw_fd(&self) -> RawFd {
        self.fd
    }
}
