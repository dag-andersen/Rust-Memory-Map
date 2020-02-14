use core::fmt;
use crate::{Entry, Utils};
use memmap::MmapMut;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub struct Node {
    pub ip: u32,
    pub value: usize,
}

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry) {
}

pub fn find_value(ip: u32) -> Option<[u8; 32]> {
    None
}
