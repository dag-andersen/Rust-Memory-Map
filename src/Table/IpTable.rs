use crate::{IP_TABLE, NAME_TABLE, u32Size, Entry, FileGenerator, Table};
use crate::Table::{NameTable, IpTable};
use crate::Utils;
use memmap::MmapMut;
use std::fs;

pub fn get_name_on_map(ip: u32, lookup_table: &MmapMut, ip_table: &MmapMut) -> Option<String> {
    let ip_address = ip as usize * u32Size;

    let addr = &ip_table[ip_address..ip_address + u32Size];
    let index = unsafe { *Utils::bytes_to_type::<u32>(addr) };

    if index == 0 { return None }
    let index = index as usize -1; // -1 because we use 0 for tracking if there is no value reference

    NameTable::get_name(&lookup_table, index)
}

pub fn place_entry(mmap: &mut MmapMut, entry: &Entry, value: u32) {
    for ip in entry.min_ip..entry.max_ip+1 {
        Utils::place_item_raw(mmap, ip as usize * u32Size, &(value+1)); // +1 because we use 0 for tracking if there is no value reference
    }
    //mmap.flush();
}

#[test]
fn place_entry_and_get_name() {
    fs::remove_file(IP_TABLE);
    fs::remove_file(NAME_TABLE);

    let mut lookup_table = Table::gen_lookup_table();
    let mut ip_table = Table::gen_ip_table();

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
    let entry = Entry { min_ip: 20, max_ip: 20, name: name3.to_string() };
    IpTable::place_entry(&mut ip_table, &entry, courser as u32);
    courser = NameTable::place_name(&mut lookup_table, courser, entry.name.as_bytes());
    //println!("{:?}",&lookup_table[0..50]);
    //println!("{:?}",&ip_table[0..200]);

    let out_name0 = get_name_on_map(0, &lookup_table, &ip_table);
    let out_name1 = get_name_on_map(5, &lookup_table, &ip_table);
    let out_name2 = get_name_on_map(9, &lookup_table, &ip_table);
    let out_name3 = get_name_on_map(20, &lookup_table, &ip_table);
    assert!(out_name0.is_some());
    assert!(out_name1.is_some());
    assert!(out_name2.is_some());
    assert!(out_name3.is_some());
    assert_eq!(out_name0.unwrap(),name1);
    assert_eq!(out_name1.unwrap(),name1);
    assert_eq!(out_name2.unwrap(),name2);
    assert_eq!(out_name3.unwrap(),name3);

    let out_name4 = get_name_on_map(40,&lookup_table,&ip_table);
    assert!(out_name4.is_none());

    fs::remove_file(IP_TABLE);
    fs::remove_file(NAME_TABLE);
}