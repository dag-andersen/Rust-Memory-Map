use crate::{u32Size, Entry, FileGenerator, Table, PayloadMap, u64Size};
use crate::Table::{IpTable, PATH, PAYLOAD};
use crate::Utils;
use memmap::MmapMut;
use std::fs;

pub fn place_entry(mmap: &mut MmapMut, entry: &Entry, value: u64) {
    for ip in entry.min_ip..entry.max_ip+1 {
        Utils::place_item_raw(mmap, ip as usize * u64Size, &(value + 1)); // +1 because we use 0 for tracking if there is no value reference
    }
}

pub fn get_payload_ptr(ip: u32, ip_table: &MmapMut) -> Option<u64> {
    let ip_address = ip as usize * u64Size;
    let addr = &ip_table[ip_address..ip_address + u64Size];
    let index = unsafe { *Utils::bytes_to_type_mut::<u64>(addr) };
    match index {
        0 => None,
        i => Some(i - 1)// -1 because we use 0 for tracking if there is no value reference
    }
}


#[test]
fn place_entry_and_get_payload() {
    fs::remove_file(PATH);
    fs::remove_file(PAYLOAD);

    let mut ip_table = super::gen_ip_table_small();

    let value = 5;

    let entry = Entry { min_ip: 0, max_ip: 5, payload: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_payload_ptr(0, &ip_table).is_some());
    assert!(get_payload_ptr(5, &ip_table).is_some());
    assert_eq!(get_payload_ptr(0, &ip_table).unwrap(), value);

    let entry = Entry { min_ip: 6, max_ip: 10, payload: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_payload_ptr(8, &ip_table).is_some());

    let entry = Entry { min_ip: 20, max_ip: 20, payload: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_payload_ptr(19, &ip_table).is_none());
    assert!(get_payload_ptr(20, &ip_table).is_some());
    assert!(get_payload_ptr(21, &ip_table).is_none());

    let entry = Entry { min_ip: 50, max_ip: 650, payload: Default::default() };
    place_entry(&mut ip_table, &entry, value);
    assert!(get_payload_ptr(50, &ip_table).is_some());
    assert!(get_payload_ptr(650, &ip_table).is_some());
    assert_eq!(get_payload_ptr(50, &ip_table).unwrap(), value);

    fs::remove_file(PATH);
    fs::remove_file(PAYLOAD);
}
