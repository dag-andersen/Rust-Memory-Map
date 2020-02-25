use crate::{TABLE1, TABLE2, u32Size, Entry, FileGenerator};
use crate::Table::{NameTable, IpTable};
use crate::Utils;
use memmap::MmapMut;
use std::fs;

pub fn get_name(ip: u32) -> Option<String> {
    let lookup_table = Utils::get_memmap(TABLE1, 4_000_000_000);
    let ip_table = Utils::get_memmap(TABLE2, 16_000_000_000);

    let ip_address = ip as usize * u32Size;

    let addr = &ip_table[ip_address..ip_address + u32Size];
    let index: u32 = unsafe { *Utils::bytes_to_type(addr) };

    if index == 0 { return None }
    let index = index as usize -1; // -1 because we use 0 for tracking if there is no value reference

    NameTable::get_name(&lookup_table, index)
}

pub fn place_entry(mmap: &mut MmapMut, entry: &Entry, value: u32) {
    for ip in entry.min_ip..entry.max_ip {
        Utils::place_item_raw(mmap, ip as usize * u32Size, &(value+1)); // +1 because we use 0 for tracking if there is no value reference
    }
}

#[test]
fn place_entry_and_get_name() {
    fs::remove_file(TABLE1);
    fs::remove_file(TABLE2);

    let mut lookup_table = Utils::get_memmap(TABLE1, 4_000_000_000);
    let mut ip_table = Utils::get_memmap(TABLE2, 16_000_000_000);

    let mut courser= 0;

    let name1 = "Hans Hansen";
    let entry = Entry { min_ip: 0, max_ip: 5, name: name1.to_string() };
    IpTable::place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut lookup_table, courser, entry.name.as_bytes());
    //println!("{:?}",&lookup_table[0..50]);
    //println!("{:?}",&ip_table[0..200]);

    let name2 = "Opvaskerne";
    let entry = Entry { min_ip: 6, max_ip: 10, name: name2.to_string() };
    IpTable::place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut lookup_table, courser, entry.name.as_bytes());
    //println!("{:?}",&lookup_table[0..50]);
    //println!("{:?}",&ip_table[0..200]);

    let name3 = "Prop";
    let entry = Entry { min_ip: 20, max_ip: 21, name: name3.to_string() };
    IpTable::place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut lookup_table, courser, entry.name.as_bytes());
    //println!("{:?}",&lookup_table[0..50]);
    //println!("{:?}",&ip_table[0..200]);

    let out_name1 = get_name(2);
    let out_name2 = get_name(9);
    let out_name3 = get_name(20);
    assert!(out_name1.is_some());
    assert!(out_name2.is_some());
    assert!(out_name3.is_some());
    assert_eq!(out_name1.unwrap(),name1);
    assert_eq!(out_name2.unwrap(),name2);
    assert_eq!(out_name3.unwrap(),name3);

    let out_name4 = get_name(40);
    assert!(out_name4.is_none());

    fs::remove_file(TABLE1);
    fs::remove_file(TABLE2);
}