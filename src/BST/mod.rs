use core::fmt;
use crate::{Entry, Utils, TREE_PATH, PayloadMap, Table, TREE_PAYLOAD, build_data_structure};
use memmap::MmapMut;
use std::fs;

mod NodeToMem;
mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub fn gen_tree_map() -> MmapMut { gen_tree_map_on_path(TREE_PATH) }
pub fn gen_tree_map_on_path(path: &str) -> MmapMut { Utils::get_memmap(path, 5_000_000_000) }

pub fn build(input: &str) { build_to_path(input, TREE_PATH) }

pub fn build_to_path(input: &str, map_path: &str) {
    fs::remove_file(map_path);
    build_data_structure(input, TREE_PAYLOAD, gen_tree_map_on_path(map_path), insert_entry)
}

pub struct Node {
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: u32,
    pub right: u32,
    pub payload_ptr: u64,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}, n: {}, min: {}, max: {}, l: {}, r: {}", &self, &self.payload_ptr, self.min_ip, self.max_ip, self.left, self.right)
    }
}

pub fn entry_to_node(entry: crate::Entry, payload_index: u64) -> Node {
    Node { min_ip: entry.min_ip, max_ip: entry.max_ip, left: 0, right: 0, payload_ptr: payload_index }
}

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry, payload_index: u64) {
    let node = entry_to_node(entry, payload_index + 1);
    Tree::insert_node(mmap, index, &node);
}

pub fn find_value(ip: u32) -> Option<String> {
    let mmap = gen_tree_map();
    let payload_map = PayloadMap::gen_payload_map_from_path(TREE_PAYLOAD);
    find_value_on_map(ip,&mmap, &payload_map)
}

pub fn find_value_on_map(ip: u32, mmap: &MmapMut, name_table: &MmapMut) -> Option<String> {
    match Tree::find_node_on_map(ip, mmap)? {
        0 => None,
        i => PayloadMap::get_payload(&name_table, i - 1)
    }
}