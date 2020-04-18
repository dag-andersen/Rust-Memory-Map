use memmap::MmapMut;
use crate::{Utils, TREE_PATH};
use crate::Tree::{NODE_SIZE, Node};
use std::fs;

fn node_from_bytes(slice: &[u8]) -> &mut Node { unsafe { Utils::bytes_to_type_mut(slice) } }

fn node_to_bytes(node: &Node) -> &[u8] { unsafe { Utils::any_as_u8_slice(node) } }

pub fn get_node<'a>(mmap: &'a MmapMut, index: usize) -> &'a mut Node {
    get_node_raw(mmap,index*NODE_SIZE)
}

fn get_node_raw<'a>(mmap: &'a MmapMut, offset: usize) -> &'a mut Node {
    let byte_map = &mmap[offset..(offset+NODE_SIZE)];
    node_from_bytes(&byte_map)
}

pub fn place_node(mmap: & mut MmapMut, index: usize, node: & Node) {
    Utils::place_item_raw(mmap,index * NODE_SIZE,node);
}


#[test]
fn test_correct_placement() {
    fs::remove_file(TREE_PATH);
    let mut name: u64 = 5;

    let node1 = super::Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: Default::default() };
    let node2 = super::Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name };

    let mut first_map = Utils::get_memmap(TREE_PATH, 300000000);
    place_node(& mut first_map, 0, &node1);
    place_node(& mut first_map, 1, &node2);

    let another_map = Utils::get_memmap(TREE_PATH, 300000000);
    let getnode = get_node(&another_map, 1);

    assert_eq!(name, getnode.name);
}
