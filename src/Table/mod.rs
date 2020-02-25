use core::fmt;
use crate::{Entry, Utils, TABLE1, TABLE2};
use memmap::MmapMut;
use std::fs;
use crate::Table;

mod IpTable;
pub mod NameTable;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub struct Node {
    pub ip: u32,
    pub value: usize,
}

pub fn insert_entry<'a, I>(vals: I)
    where
        I: IntoIterator<Item = Entry>,
{
    fs::remove_file(TABLE1);
    fs::remove_file(TABLE2);

    let mut lookup_table = Utils::get_memmap(TABLE1, 4_000_000_000);
    let mut ip_table = Utils::get_memmap(TABLE2, 16_000_000_000);

    let mut courser= 0;
    for entry in vals {
        IpTable::place_entry(&mut ip_table, &entry, courser as u32);
        courser = NameTable::place_name(&mut lookup_table, courser, entry.name.as_bytes());
    }
}

pub fn find_value(ip: u32) -> Option<String> {
    IpTable::get_name(ip)
}
