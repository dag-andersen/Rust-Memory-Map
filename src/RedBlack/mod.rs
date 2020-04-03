use core::fmt;
use crate::{Entry, Utils, REDBLACK_PATH, NameTable, Table, REDBLACK_PAYLOAD};
use memmap::MmapMut;
use crate::RedBlack::Tree::root_index;

mod NodeToMem;
mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub fn reset_root_index() { Tree::reset_root_index() }
pub fn save_root_node(map_path: &str) { Tree::save_root_node(map_path) }
pub fn load_root_node(map_path: &str) { Tree::load_root_node(map_path) }

pub fn gen_tree_map() -> MmapMut { gen_tree_map_on_path(REDBLACK_PATH) }
pub fn gen_tree_map_on_path(path: &str) -> MmapMut { Utils::get_memmap(path, 7_000_000_000) }

pub struct Node {
    pub red: bool,
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: usize,
    pub right: usize,
    pub parent: usize,
    pub name: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}, n: {}, min: {}, max: {}, l: {}, r: {}, p: {}", &self, &self.name, self.min_ip, self.max_ip, self.left, self.right, self.parent)
    }
}

pub fn entry_to_node(entry: crate::Entry, index: usize) -> Node {
    Node { red: true, min_ip: entry.min_ip, max_ip: entry.max_ip, left: 0, right: 0, parent: 0, name: index }
}

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry, name_index: usize) {
    let mut node: Node = entry_to_node(entry, name_index + 1);
    Tree::insert_node(mmap, index + 1, &mut node);
}

pub fn find_value(ip: u32) -> Option<String> {
    let mmap = gen_tree_map();
    let name_table = NameTable::gen_name_table_from_path(REDBLACK_PAYLOAD);
    find_value_on_map(ip,&mmap, &name_table)
}

pub fn find_value_on_map(ip: u32, mmap: &MmapMut, name_table: &MmapMut) -> Option<String> {
    match Tree::find_node_on_map(ip, mmap)? {
        0 => None,
        i => NameTable::get_name(&name_table, i - 1)
    }
}
