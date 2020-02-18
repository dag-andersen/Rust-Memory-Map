use crate::{Utils, usizeSize};
use memmap::{MmapMut, MmapOptions};

pub fn place_name(mmap: &mut MmapMut, offset: usize, name: &[u8]) -> usize {
    let len = name.len();
    Utils::place_item_raw(mmap, offset, &len);
    mmap[offset+usizeSize..offset+usizeSize+len].copy_from_slice(name);
    offset + usizeSize + len
}

pub fn get_name(mmap: &MmapMut, offset: usize) {
    let namesize: usize = unsafe { *Utils::bytes_to_type(&mmap[offset..(offset+usizeSize)]) };
    let nameAsBytes = &mmap[offset+usizeSize..offset+usizeSize+namesize];
    let name = std::str::from_utf8(nameAsBytes).expect("bad formatting");
    println!("crazy test: {}",name);
}