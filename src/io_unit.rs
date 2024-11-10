use crate::BenioFile;

pub struct IoUnit<'file, const LEN: usize> {
    index: usize,
    file: &'file BenioFile,

    offset: u64,
    len: u64,
    buf: [u8; LEN],
}
