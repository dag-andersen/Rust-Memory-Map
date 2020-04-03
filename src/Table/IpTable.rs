use crate::{TABLE_PATH, u32Size, Entry, FileGenerator, Table, NameTable, TABLE_PAYLOAD};
use crate::Table::{IpTable};
use crate::Utils;
use memmap::MmapMut;
use std::fs;

pub fn place_entry(mmap: &mut MmapMut, entry: &Entry, value: u32) {
    for ip in entry.min_ip..entry.max_ip+1 {
        Utils::place_item_raw(mmap, ip as usize * u32Size, &(value + 1)); // +1 because we use 0 for tracking if there is no value reference
    }
}

pub fn get_name_on_map(ip: u32, ip_table: &MmapMut) -> Option<u32> {
    let ip_address = ip as usize * u32Size;
    let addr = &ip_table[ip_address..ip_address + u32Size];
    let index = unsafe { *Utils::bytes_to_type_mut::<u32>(addr) };
    match index {
        0 => None,
        i => Some(i - 1)// -1 because we use 0 for tracking if there is no value reference
    }
}


#[test]
fn place_entry_and_get_name() {
    fs::remove_file(TABLE_PATH);
    fs::remove_file(TABLE_PAYLOAD);

    let mut ip_table = super::gen_ip_table();

    let value = 5;

    let entry = Entry { min_ip: 0, max_ip: 5, name: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_name_on_map(0, &ip_table).is_some());
    assert!(get_name_on_map(5, &ip_table).is_some());
    assert_eq!(get_name_on_map(0, &ip_table).unwrap(), value);

    let entry = Entry { min_ip: 6, max_ip: 10, name: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_name_on_map(8, &ip_table).is_some());

    let entry = Entry { min_ip: 20, max_ip: 20, name: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_name_on_map(19, &ip_table).is_none());
    assert!(get_name_on_map(20, &ip_table).is_some());
    assert!(get_name_on_map(21, &ip_table).is_none());

    let entry = Entry { min_ip: 50, max_ip: 650, name: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_name_on_map(50, &ip_table).is_some());
    assert!(get_name_on_map(650, &ip_table).is_some());
    assert_eq!(get_name_on_map(50, &ip_table).unwrap(), value);

    fs::remove_file(TABLE_PATH);
    fs::remove_file(TABLE_PAYLOAD);
}
