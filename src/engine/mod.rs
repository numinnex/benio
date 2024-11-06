use crate::BenioFile;
use std::io;

pub mod io_uring;

pub trait Engine {
    fn new_file(&mut self, path: &str) -> io::Result<BenioFile>;
    fn close_file(&mut self, file: BenioFile) -> io::Result<()>;
}
