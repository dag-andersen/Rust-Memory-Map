use crate::{TABLE1, TABLE2, u32Size};
use crate::Table::TableLookup;
use crate::Utils;

pub fn get_name(ip: u32) -> String {
    let lookup_table = Utils::get_memmap(TABLE1, 4_000_000_000);
    let ip_table = Utils::get_memmap(TABLE2, 16_000_000_000);

    let ip_adresse = u32Size * ip as usize;

    let addr = &ip_table[ip_adresse..ip_adresse+4];
    let index: u32 = unsafe { *Utils::bytes_to_type(addr) };

    TableLookup::get_name(&ip_table, index as usize)
}