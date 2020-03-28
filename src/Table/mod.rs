use core::fmt;
use crate::{Entry, Utils, IP_TABLE, NAME_TABLE, NameTable};
use memmap::MmapMut;
use std::fs;
use crate::Table;

mod IpTable;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub struct Node {
    pub ip: u32,
    pub value: usize,
}

pub fn insert_entry(ip_table: &mut MmapMut, entry: Entry, courser: usize) {
    IpTable::place_entry(ip_table, &entry, courser as u32);
}

pub fn gen_ip_table() -> MmapMut { gen_ip_table_from_path(IP_TABLE) }
pub fn gen_ip_table_from_path(path: &str) -> MmapMut { Utils::get_memmap(path, 20_000_000_000) }

pub fn find_value(ip: u32) -> Option<String> {
    let name_table = NameTable::gen_name_table();
    let ip_table = gen_ip_table();
    find_value_on_map(ip, &ip_table,&name_table)
}

pub fn find_value_on_map(ip: u32, ip_table: &MmapMut, name_table: &MmapMut) -> Option<String> {
    let index = IpTable::get_name_on_map(ip,ip_table)?;
    NameTable::get_name(&name_table, index)
}
