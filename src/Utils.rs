use memmap::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io::{SeekFrom, Write, Seek};
use crate::{Tree, Entry};
use regex::bytes::Regex;

pub(crate) fn insert_array_in_array(one: & mut [u8; 32], two: &[u8])  {
    for (place, data) in one.iter_mut().zip(two.iter()) {
        *place = *data
    }
}

pub(crate) fn get_entry_for_line(ip_regex: &Regex, name_regex: &Regex, l: String) -> Option<Entry> {

    let min_ip_match = ip_regex.find(l.as_bytes()).expect("didnt find min ip");
    let max_ip_match = ip_regex.find_at(l.as_bytes(), min_ip_match.end()).expect("didnt find max ip");
    let name_match = name_regex.find_at(l.as_bytes(), max_ip_match.end()).expect("didnt find name");

    //println!("min:{}- max:{}- name:{}", &l[min_ip_match.range()], &l[max_ip_match.range()], &l[name_match.range()]);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, name_match.as_bytes());

    let min_ip = get_u32_for_ip(&l[min_ip_match.range()])?;
    let max_ip = get_u32_for_ip(&l[max_ip_match.range()])?;

    Some(Entry { min_ip, max_ip, name })
}

pub(crate) fn get_u32_for_ip(v: &str ) -> Option<u32> {
    let v: Vec<&str> = v.split('.').collect();
    if v.len() != 4 { return None }
    let mut min_array: [u8; 4] = Default::default();
    for i in 0..v.len() {
        min_array[i] = match v[i].parse() {
            Ok(n) => n,
            Err(e) => return None
        }
    }
    //println!("IP?{}.{}.{}.{}",min_array[0],min_array[1],min_array[2],min_array[3]);
    Some(u32::from_be_bytes(min_array))
}

pub(crate) fn entry_to_node(entry: crate::Entry) -> Tree::Node {
    Tree::Node { min_ip: entry.min_ip, max_ip: entry.max_ip, left: 0, right: 0, name: entry.name }
}

pub(crate) fn get_memmap(source: &str, size: u64) -> MmapMut {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(source)
        .expect("Unable to open file");
    file.seek(SeekFrom::Start(size)).unwrap();
    file.write_all(&[0]).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut mmap = unsafe { MmapOptions::new().map_mut( & file).unwrap() };
    mmap
}

pub(crate) unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

pub(crate) unsafe fn bytes_to_typed<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}

pub(crate) fn place_item_raw<T>(mmap: & mut MmapMut, offset: usize, t: &T) {
    let bytes = unsafe { any_as_u8_slice(t) };
    mmap[offset..(offset+bytes.len())].copy_from_slice(bytes);
}