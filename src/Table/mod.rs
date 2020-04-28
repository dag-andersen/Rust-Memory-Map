use core::fmt;
use crate::{Entry, Utils, TABLE_PATH, PayloadMap, TABLE_PAYLOAD, build_data_structure};
use memmap::MmapMut;
use std::fs;
use crate::Table;

mod IpTable;

pub fn gen_ip_table() -> MmapMut { gen_ip_table_from_path(TABLE_PATH) }
pub fn gen_ip_table_from_path(path: &str) -> MmapMut { Utils::get_memmap(path, 40_000_000_000) }
pub fn gen_ip_table_small() -> MmapMut { Utils::get_memmap(TABLE_PATH, 500_000_000) }

pub fn build(input: &str) { build_to_path(input, TABLE_PATH) }

fn build_to_path(input: &str, ip_table: &str) {
    fs::remove_file(ip_table);
    build_data_structure(input, TABLE_PAYLOAD, Table::gen_ip_table_from_path(ip_table), Table::insert_entry)
}

pub fn build_to_path_small(input: &str) {
    fs::remove_file(TABLE_PATH);
    build_data_structure(input, TABLE_PAYLOAD, Table::gen_ip_table_small(), Table::insert_entry)
}

pub fn insert_entry(ip_table: &mut MmapMut, index: usize, entry: Entry, courser: u64) {
    IpTable::place_entry(ip_table, &entry, courser);
}

pub fn find_value(ip: u32) -> Option<String> {
    let name_table = PayloadMap::gen_payload_map_from_path(TABLE_PAYLOAD);
    let ip_table = gen_ip_table();
    find_value_on_map(ip, &ip_table,&name_table)
}

pub fn find_value_small(ip: u32) -> Option<String> {
    let name_table = PayloadMap::gen_payload_map_from_path(TABLE_PAYLOAD);
    let ip_table = gen_ip_table_small();
    find_value_on_map(ip, &ip_table,&name_table)
}

pub fn find_value_on_map(ip: u32, ip_table: &MmapMut, name_table: &MmapMut) -> Option<String> {
    let index = IpTable::get_payload_ptr(ip, ip_table)?;
    PayloadMap::get_payload(&name_table, index)
}
