use ::io_uring::cqueue;
use crate::{io_unit::IoUnit, BenioFile};
use std::io;

pub mod io_uring;

// Potential extra methods to attach.
// - Init
// - Submit
// - Commit
// - File Stat
pub trait Engine {
    fn init(depth: u32) -> Self;
    fn open_file(&mut self, path: &str) -> io::Result<BenioFile>;
    fn close_file(&mut self, file: BenioFile) -> io::Result<()>;
    fn queue<const LEN: usize>(io_u: IoUnit<LEN>) -> io::Result<()>;
    fn commit() -> io::Result<usize>;
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
