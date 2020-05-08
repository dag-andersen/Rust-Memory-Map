use core::fmt;
use crate::{Entry, Utils, PayloadMap, build_data_structure};
use memmap::MmapMut;
use std::fs;
use crate::Table;

mod IpTable;

pub const PATH:               &str    = "testdata/out/table/IP_TABLE.txt";
pub const PAYLOAD:            &str    = "testdata/out/table/NAME_TABLE.txt";

pub fn gen_ip_table() -> MmapMut { gen_ip_table_from_path(PATH) }
pub fn gen_ip_table_from_path(path: &str) -> MmapMut { Utils::get_memmap(path, 40_000_000_000) }
pub fn gen_ip_table_small() -> MmapMut { Utils::get_memmap(PATH, 500_000_000) }

pub fn build(input: &str) { build_to_path(input, PATH) }

fn build_to_path(input: &str, ip_table: &str) {
    fs::remove_file(ip_table);
    build_data_structure(input, PAYLOAD, Table::gen_ip_table_from_path(ip_table), Table::insert_entry)
}

pub fn build_to_path_small(input: &str) {
    fs::remove_file(PATH);
    build_data_structure(input, PAYLOAD, Table::gen_ip_table_small(), Table::insert_entry)
}

pub fn insert_entry(ip_table: &mut MmapMut, index: usize, entry: Entry, courser: u64) {
    IpTable::place_entry(ip_table, &entry, courser);
}

pub fn find_value(ip: u32) -> Option<String> {
    let payload_map = PayloadMap::gen_payload_map_from_path(PAYLOAD);
    let ip_table = gen_ip_table();
    find_value_on_map(ip, &ip_table,&payload_map)
}

pub fn find_value_small(ip: u32) -> Option<String> {
    let payload_map = PayloadMap::gen_payload_map_from_path(PAYLOAD);
    let ip_table = gen_ip_table_small();
    find_value_on_map(ip, &ip_table,&payload_map)
}

pub fn find_value_on_map(ip: u32, ip_table: &MmapMut, payload_map: &MmapMut) -> Option<String> {
    let index = IpTable::get_payload_ptr(ip, ip_table)?;
    PayloadMap::get_payload(&payload_map, index)
}
