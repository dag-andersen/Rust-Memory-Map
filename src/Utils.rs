use memmap::{MmapMut, MmapOptions};
use std::fs::{OpenOptions, File};
use std::io::{SeekFrom, Write, Seek, BufReader};
use crate::{BST, Entry};
use regex::bytes::Regex;

pub(crate) fn get_entry_for_line(ip_regex: &Regex, name_regex: &Regex, l: &String) -> Option<Entry> {
    let min_ip_match = ip_regex.find(l.as_bytes()).expect("didnt find min ip");
    let max_ip_match = ip_regex.find_at(l.as_bytes(), min_ip_match.end()).expect("didnt find max ip");
    let name_match = name_regex.find_at(l.as_bytes(), max_ip_match.end()).expect("didnt find name");

    let min_ip = get_u32_for_ip(&l[min_ip_match.range()])?;
    let max_ip = get_u32_for_ip(&l[max_ip_match.range()])?;

    Some(Entry { min_ip, max_ip, payload: String::from(&l.as_str()[name_match.range()]) })
}

pub(crate) fn get_u32_for_ip(s: &str) -> Option<u32> {
    let v: Vec<&str> = s.split('.').collect();
    let len = v.len();
    if len != 4 { return None }
    let mut acc: u32 = 0;
    for i in 0..len {
        match v[i].parse::<u8>() {
            Ok(n) => acc |= (n as u32) << ((len-1-i) * 8) as u32,
            Err(e) => return None
        };
    }
    Some(acc)
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

pub(crate) unsafe fn bytes_to_type<T>(slice: &[u8]) -> &T {
    std::slice::from_raw_parts(slice.as_ptr() as *const T, std::mem::size_of::<T>())
        .get(0)
        .unwrap()
}

pub(crate) unsafe fn bytes_to_type_mut<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}

pub(crate) fn place_item_raw<T>(mmap: & mut MmapMut, offset: usize, t: &T) {
    let bytes = unsafe { any_as_u8_slice(t) };
    mmap[offset..(offset+bytes.len())].copy_from_slice(bytes);
}

pub fn get_buffer(file: &str) -> BufReader<std::fs::File> {
    BufReader::new(File::open(file).expect("could not find file"))
}

pub fn make_needed_folders() {
    std::fs::create_dir_all("testdata/out/tree").expect("couldn't create folder");
    std::fs::create_dir_all("testdata/out/redblack").expect("couldn't create folder");
    std::fs::create_dir_all("testdata/out/table").expect("couldn't create folder");
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