use memmap::MmapMut;
use crate::{NODE_SIZE, Node};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

fn node_from_bytes(slice: &[u8]) -> &mut Node { unsafe { bytes_to_typed(slice) } }

unsafe fn bytes_to_typed<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}

fn node_to_bytes(node: &Node) -> &[u8] { unsafe { any_as_u8_slice(node) } }

pub fn get_node<'a>(mmap: &'a MmapMut, index: usize) -> &'a mut Node {
    get_node_raw(mmap,index*NODE_SIZE)
}

fn get_node_raw<'a>(mmap: &'a MmapMut, offset: usize) -> &'a mut Node {
    let byte_map = &mmap[offset..(offset+NODE_SIZE)];
    let node = node_from_bytes(&byte_map);
    node
}

pub fn place_item(mmap: & mut MmapMut, index: usize, node: & Node) {
    place_item_raw(mmap,index * NODE_SIZE,node);
}

fn place_item_raw(mmap: & mut MmapMut, offset: usize, node: & Node,) {
    let bytes = node_to_bytes(node);
    mmap[offset..(offset+bytes.len())].copy_from_slice(bytes);
}