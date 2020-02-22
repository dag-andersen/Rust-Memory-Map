use core::fmt;
use crate::{Entry, Utils};
use memmap::MmapMut;

mod Table;
pub mod TableLookup;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub struct Node {
    pub ip: u32,
    pub value: usize,
}

pub fn insert_entry(mmap1: &mut MmapMut, mmap2: &mut MmapMut, index: usize, entry: Entry) {
}

pub fn find_value(ip: u32) -> String {
    Table::get_name(ip)
}
