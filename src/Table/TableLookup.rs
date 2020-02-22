use crate::{Utils, usizeSize, u8Size, Entry, u32Size};
use memmap::{MmapMut, MmapOptions};

pub fn place_name(mmap: &mut MmapMut, offset: usize, name: &[u8]) -> usize {
    let len = name.len();
    //mmap[offset..offset+u8Size].copy_from_slice(&[len as u8]);
    Utils::place_item_raw(mmap, offset, &(len as u8));
    mmap[offset+u8Size..offset+u8Size+len].copy_from_slice(name);
    offset + u8Size + len
}

pub fn get_name(mmap: &MmapMut, offset: usize) -> String {
    let name_size: u8 = unsafe { *Utils::bytes_to_type(&mmap[offset..(offset+u8Size)]) };
    let nameAsBytes = &mmap[offset+u8Size..offset+u8Size+(name_size as usize)];
    std::str::from_utf8(nameAsBytes).expect("bad formatting").to_string()
}

pub fn place_entry(mmap: &mut MmapMut, entry: &Entry, value: u32) {
    for ip in entry.min_ip..entry.max_ip {
        Utils::place_item_raw(mmap, ip as usize * u32Size, &value);
    }
}

#[test]
fn test() {
    let scr = "ThisWillBeDeleted";
    let mmap = Utils::get_memmap(scr, 1_000);
    //place_name(&mmap, )

}