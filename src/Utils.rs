use memmap::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io::{SeekFrom, Write, Seek};

pub(crate) fn insert_array_in_array(one: & mut [u8; 32], two: &[u8])  {
    for (place, data) in one.iter_mut().zip(two.iter()) {
        *place = *data
    }
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