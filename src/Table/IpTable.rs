use crate::{TABLE_PATH, u32Size, Entry, FileGenerator, Table, NameTable};
use crate::Table::{IpTable};
use crate::Utils;
use memmap::MmapMut;
use std::fs;

pub fn place_entry(mmap: &mut MmapMut, entry: &Entry, value: u32) {
    for ip in entry.min_ip..entry.max_ip+1 {
        Utils::place_item_raw(mmap, ip as usize * u32Size, &(value+1)); // +1 because we use 0 for tracking if there is no value reference
    }
}

pub fn get_name_on_map(ip: u32, ip_table: &MmapMut) -> Option<usize> {
    let ip_address = ip as usize * u32Size;
    let addr = &ip_table[ip_address..ip_address + u32Size];
    let index = unsafe { *Utils::bytes_to_type_mut::<u32>(addr) };
    if index == 0 { return None }
    Some(index as usize - 1) // -1 because we use 0 for tracking if there is no value reference
}

/*
#[test]
fn place_entry_and_get_name() {
    fs::remove_file(IP_TABLE);
    fs::remove_file(NAME_TABLE);

    let mut name_table = NameTable::gen_name_table();
    let mut ip_table = super::gen_ip_table();

    let mut courser= 0;

    let name1 = "Hans Hansen";
    let entry = Entry { min_ip: 0, max_ip: 5, name: name1.to_string() };
    place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut name_table, courser, entry.name.as_bytes());

    let name2 = "Opvaskerne";
    let entry = Entry { min_ip: 6, max_ip: 10, name: name2.to_string() };
    place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut name_table, courser, entry.name.as_bytes());

    let name3 = "Prop";
    let entry = Entry { min_ip: 20, max_ip: 20, name: name3.to_string() };
    place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut name_table, courser, entry.name.as_bytes());

    let name4 = "HejMedDig";
    let entry = Entry { min_ip: 50, max_ip: 650, name: name4.to_string() };
    place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut name_table, courser, entry.name.as_bytes());

    let out_name0 = get_name_on_map(0, &name_table, &ip_table);
    let out_name1 = get_name_on_map(5, &name_table, &ip_table);
    let out_name2 = get_name_on_map(9, &name_table, &ip_table);
    let out_name3 = get_name_on_map(20, &name_table, &ip_table);
    let out_name4 = get_name_on_map(50, &name_table, &ip_table);
    let out_name5 = get_name_on_map(144, &name_table, &ip_table);
    let out_name6 = get_name_on_map(650, &name_table, &ip_table);
    assert!(out_name0.is_some());
    assert!(out_name1.is_some());
    assert!(out_name2.is_some());
    assert!(out_name3.is_some());
    assert!(out_name4.is_some());
    assert!(out_name5.is_some());
    assert!(out_name6.is_some());
    assert_eq!(out_name0.unwrap(),name1);
    assert_eq!(out_name1.unwrap(),name1);
    assert_eq!(out_name2.unwrap(),name2);
    assert_eq!(out_name3.unwrap(),name3);
    assert_eq!(out_name4.unwrap(),name4);
    assert_eq!(out_name5.unwrap(),name4);
    assert_eq!(out_name6.unwrap(),name4);

    let out_name1 = get_name_on_map(40, &name_table, &ip_table);
    let out_name2 = get_name_on_map(21, &name_table, &ip_table);
    let out_name3 = get_name_on_map(651, &name_table, &ip_table);
    assert!(out_name1.is_none());
    assert!(out_name2.is_none());
    assert!(out_name3.is_none());

    fs::remove_file(IP_TABLE);
    fs::remove_file(NAME_TABLE);
}
*/
