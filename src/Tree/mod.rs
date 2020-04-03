use core::fmt;
use crate::{Entry, Utils, TREE_PATH, NameTable, Table, TREE_PAYLOAD};
use memmap::MmapMut;

mod NodeToMem;
mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub fn entry_to_node(entry: crate::Entry, name_index: usize) -> Node {
    Node { min_ip: entry.min_ip, max_ip: entry.max_ip, left: 0, right: 0, name: name_index }
}

pub struct Node {
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: usize,
    pub right: usize,
    pub name: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}, n: {}, min: {}, max: {}, l: {}, r: {}", &self, &self.name, self.min_ip, self.max_ip, self.left, self.right)
    }
}

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry, name_index: usize) {
    let node = entry_to_node(entry, name_index + 1);
    Tree::insert_node(mmap, index, &node);
}

pub fn gen_tree_map() -> MmapMut { gen_tree_map_on_path(TREE_PATH) }
pub fn gen_tree_map_on_path(path: &str) -> MmapMut { Utils::get_memmap(path, 5_000_000_000) }

pub fn find_value(ip: u32) -> Option<String> {
    let mmap = gen_tree_map();
    let name_table = NameTable::gen_name_table_from_path(TREE_PAYLOAD);
    find_value_on_map(ip,&mmap, &name_table)
}

pub fn find_value_on_map(ip: u32, mmap: &MmapMut, name_table: &MmapMut) -> Option<String> {
    match Tree::find_node_on_map(ip, mmap)? {
        0 => None,
        i => NameTable::get_name(&name_table, i - 1)
    }
}