use core::fmt;
use crate::{Entry, Utils, PayloadMap, Table, build_data_structure};
use memmap::MmapMut;
use crate::RedBlack::Tree::root_index;
use std::fs;

pub const PRINT_PATH:      &str    = "testdata/out/redblack/tree_print.txt";
pub const PATH:            &str    = "testdata/out/redblack/map.txt";
pub const PAYLOAD:         &str    = "testdata/out/redblack/NAME_TABLE.txt";

mod NodeToMem;
mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub fn reset_root_index() { Tree::reset_root_index() }

pub fn save_root_node() { Tree::save_root_node(PATH) }
pub fn save_root_node_to_path(map_path: &str) { Tree::save_root_node(map_path) }

pub fn load_root_node() { unsafe { root_index = Tree::get_root_node(PATH)} }
pub fn load_root_node_from_path(map_path: &str) { unsafe { root_index = Tree::get_root_node(map_path)} }

pub fn gen_tree_map() -> MmapMut { gen_tree_map_on_path(PATH) }
pub fn gen_tree_map_on_path(path: &str) -> MmapMut { Utils::get_memmap(path, 7_500_000_000) }

pub fn build(input: &str) { build_to_path(input, PATH) }

fn build_to_path(input: &str, map_path: &str) {
    reset_root_index();
    fs::remove_file(map_path);
    build_data_structure(input, PAYLOAD, gen_tree_map_on_path(map_path), insert_entry);
    save_root_node_to_path(map_path);
}

pub struct Node {
    pub red: bool,
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: u32,
    pub right: u32,
    pub parent: u32,
    pub payload_ptr: u64,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}, n: {}, min: {}, max: {}, l: {}, r: {}, p: {}", &self, &self.payload_ptr, self.min_ip, self.max_ip, self.left, self.right, self.parent)
    }
}

pub fn entry_to_node(entry: crate::Entry, index: u64) -> Node {
    Node { red: true, min_ip: entry.min_ip, max_ip: entry.max_ip, left: 0, right: 0, parent: 0, payload_ptr: index }
}

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry, payload_index: u64) {
    let mut node: Node = entry_to_node(entry, payload_index + 1);
    Tree::insert_node(mmap, index + 1, &mut node);
}

pub fn find_value(ip: u32) -> Option<String> {
    let mmap = gen_tree_map();
    let payload_map = PayloadMap::gen_payload_map_from_path(PAYLOAD);
    find_value_on_map(ip,&mmap, &payload_map)
}

pub fn find_value_on_map(ip: u32, mmap: &MmapMut, payload_map: &MmapMut) -> Option<String> {
    match Tree::find_node_on_map(ip, mmap)? {
        0 => None,
        i => PayloadMap::get_payload(&payload_map, i - 1)
    }
}
