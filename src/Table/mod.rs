use core::fmt;
use crate::{Entry, Utils, TABLE_PATH, NameTable, TABLE_PAYLOAD};
use memmap::MmapMut;
use std::fs;
use crate::Table;

mod IpTable;

pub fn insert_entry(ip_table: &mut MmapMut, index: usize, entry: Entry, courser: u64) {
    IpTable::place_entry(ip_table, &entry, courser);
}

pub fn gen_ip_table() -> MmapMut { gen_ip_table_from_path(TABLE_PATH) }
pub fn gen_ip_table_from_path(path: &str) -> MmapMut { Utils::get_memmap(path, 40_000_000_000) }

pub fn find_value(ip: u32) -> Option<String> {
    let name_table = NameTable::gen_name_table_from_path(TABLE_PAYLOAD);
    let ip_table = gen_ip_table();
    find_value_on_map(ip, &ip_table,&name_table)
}

pub fn find_value_on_map(ip: u32, ip_table: &MmapMut, name_table: &MmapMut) -> Option<String> {
    let index = IpTable::get_name_on_map(ip,ip_table)?;
    NameTable::get_name(&name_table, index)
}
