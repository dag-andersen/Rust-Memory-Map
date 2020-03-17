use core::fmt;
use crate::{Entry, Utils, IP_TABLE, NAME_TABLE};
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

pub fn insert_entry<'a, I: IntoIterator<Item=Entry>>(vals: I)
{
    insert_entry_on_path(vals, IP_TABLE, NAME_TABLE);
}

pub fn insert_entry_on_path<'a, I>(vals: I, ip_talbe: &str, name_table: &str)
    where
        I: IntoIterator<Item = Entry>,
{
    fs::remove_file(ip_talbe);
    fs::remove_file(name_table);

    let mut lookup_table = gen_lookup_table_from_path(name_table);
    let mut ip_table = gen_ip_table_from_path(ip_talbe);

    let mut counter = 0;
    let mut courser= 0;
    for entry in vals {
        if counter % 500_000 == 0 { println!("Table: pushed {} lines", counter)}
        IpTable::place_entry(&mut ip_table, &entry, courser as u32);
        courser = NameTable::place_name(&mut lookup_table, courser, entry.name.as_bytes());
        counter += 1;
    }
}

pub fn gen_lookup_table() -> MmapMut { gen_lookup_table_from_path(NAME_TABLE) }
pub fn gen_lookup_table_from_path(path: &str) -> MmapMut { Utils::get_memmap(path, 40_000_000_000) }

pub fn gen_ip_table() -> MmapMut { gen_ip_table_from_path(IP_TABLE) }
pub fn gen_ip_table_from_path(path: &str) -> MmapMut { Utils::get_memmap(path, 20_000_000_000) }

pub fn find_value(ip: u32) -> Option<String> {
    let lookup_table = gen_lookup_table();
    let ip_table = gen_ip_table();
    find_value_on_map(ip, &lookup_table,&ip_table)
}

pub fn find_value_on_map(ip: u32, lookup_table: &MmapMut, ip_table: &MmapMut) -> Option<String> {
    IpTable::get_name_on_map(ip,lookup_table,ip_table)
}
