use ::io_uring::cqueue;

use crate::BenioFile;
use std::io;

pub mod io_uring;

pub trait Engine {
    fn new_file(&mut self, path: &str) -> io::Result<BenioFile>;
    fn close_file(&mut self, file: BenioFile) -> io::Result<()>;
}

#[inline]
fn resultify(cqe: &cqueue::Entry) -> io::Result<i32> {
    let res = cqe.result();

    if res >= 0 {
        Ok(res)
    } else {
        Err(io::Error::from_raw_os_error(-res))
    }
}
