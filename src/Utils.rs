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

pub(crate) fn get_entry_for_line(ip_regex: &Regex, name_regex: &Regex, l: &String) -> Option<Entry> {

    let min_ip_match = ip_regex.find(l.as_bytes()).expect("didnt find min ip");
    let max_ip_match = ip_regex.find_at(l.as_bytes(), min_ip_match.end()).expect("didnt find max ip");
    let name_match = name_regex.find_at(l.as_bytes(), max_ip_match.end()).expect("didnt find name");

    //println!("min:{}- max:{}- name:{}", &l[min_ip_match.range()], &l[max_ip_match.range()], &l[name_match.range()]);

    let min_ip = get_u32_for_ip(&l[min_ip_match.range()])?;
    let max_ip = get_u32_for_ip(&l[max_ip_match.range()])?;

    Some(Entry { min_ip, max_ip, name: String::from(&l.as_str()[name_match.range()]) })
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

pub(crate) fn entry_to_node(entry: crate::Entry, index: usize) -> Tree::Node {
    Tree::Node { red: true, min_ip: entry.min_ip, max_ip: entry.max_ip, left: 0, right: 0, parent: 0, name: index }
}

pub(crate) fn get_memmap(source: &str, size: u64) -> MmapMut {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(source)
        .expect("couldn't open file");
    file.set_len(size).expect("error while setting length of file");
    unsafe { MmapOptions::new().map_mut(&file).unwrap() }
}

pub(crate) unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

pub(crate) unsafe fn bytes_to_type<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}

pub(crate) fn place_item_raw<T>(mmap: & mut MmapMut, offset: usize, t: &T) {
    let bytes = unsafe { any_as_u8_slice(t) };
    mmap[offset..(offset+bytes.len())].copy_from_slice(bytes);
}


#[test]
fn test_get_ip_for_line() {
    let ip_str = "0.0.0.132";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_some());
    assert_eq!(ip_u32.unwrap(),132);

    let ip_str = "0.0.1.1";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_some());
    assert_eq!(ip_u32.unwrap(),257);

    let ip_str = "0.0.0.300";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_none());

    let ip_str = "0.1.1";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_none());
}