use memmap::MmapMut;
use crate::{Utils};
use crate::Tree::{NODE_SIZE, Node};

fn node_from_bytes(slice: &[u8]) -> &mut Node { unsafe { Utils::bytes_to_typed(slice) } }

fn node_to_bytes(node: &Node) -> &[u8] { unsafe { Utils::any_as_u8_slice(node) } }

pub fn get_node<'a>(mmap: &'a MmapMut, index: usize) -> &'a mut Node {
    get_node_raw(mmap,index*NODE_SIZE)
}

fn get_node_raw<'a>(mmap: &'a MmapMut, offset: usize) -> &'a mut Node {
    let byte_map = &mmap[offset..(offset+NODE_SIZE)];
    node_from_bytes(&byte_map)
}

pub fn place_item(mmap: & mut MmapMut, index: usize, node: & Node) {
    Utils::place_item_raw(mmap,index * NODE_SIZE,node);
}
