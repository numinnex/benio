use super::{resultify, Engine};
use crate::BenioFile;
use io_uring::{opcode, types::Fd};
use std::io;
use std::ffi::CString;
pub struct IoUring {
    ring: io_uring::IoUring,
}

impl IoUring {
    pub fn new(queue_depth: u32) -> Self {
        todo!()
    }
}

impl Engine for IoUring {
    // SAFETY: It's up to user to make sure when this function is called
    // the SQ is empty and doesn't have any inflight ops.
    fn new_file(&mut self, path: &str) -> io::Result<BenioFile> {
        let cstring = CString::new(path).expect("Failed to convert path to CString");
        let flags = libc::O_CREAT | libc::O_RDWR | libc::O_CREAT | libc::O_DIRECT | libc::O_SYNC;
        let mode = 0o666; // Default mode
        let openat_op = opcode::OpenAt::new(Fd(libc::AT_FDCWD), cstring.as_c_str().as_ptr())
            .flags(flags)
            .mode(mode)
            .build();
        unsafe {
            self.ring
                .submission()
                .push(&openat_op)
                .expect("Failed to submit OpenAt op, queue is full");
        }
        self.ring.submit_and_wait(1)?;
        let cqe = self
            .ring
            .completion()
            .next()
            .expect("Failed to reap OpenAt op, queue is empty");
        let fd = resultify(&cqe)?;

        Ok(BenioFile { fd })
    }

    // SAFETY: It's up to user to make sure when this function is called
    // the SQ is empty and doesn't have any inflight ops.
    fn close_file(&mut self, file: BenioFile) -> io::Result<()> {
        let fd = file.raw_fd();
        let close_op = opcode::Close::new(Fd(fd)).build();
        unsafe {
            self.ring
                .submission()
                .push(&close_op)
                .expect("Failed to submit OpenAt op, queue is full");
        }
        self.ring.submit_and_wait(1)?;
        let cqe = self
            .ring
            .completion()
            .next()
            .expect("Failed to reap OpenAt op, queue is empty");
        let _ = resultify(&cqe)?;

        Ok(())
    }
}
