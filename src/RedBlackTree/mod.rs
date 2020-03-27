use core::fmt;
use crate::{Entry, Utils, TREE_PATH, NameTable, Table};
use memmap::MmapMut;
use crate::RedBlackTree::Tree::root_index;

pub mod NodeToMem;
mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub fn reset_root_index() { unsafe { root_index = 1 }; }

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

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry, name_index: usize) {
    let mut node = Utils::entry_to_node(entry, name_index);
    node.name = name_index;
    Tree::insert_node(mmap, index, &mut node);
}

pub fn gen_tree_map() -> MmapMut { gen_tree_map_on_path(TREE_PATH) }
pub fn gen_tree_map_on_path(path: &str) -> MmapMut { Utils::get_memmap(path, 20_000_000_000) }

pub fn find_value(ip: u32) -> Option<String> {
    let mmap = gen_tree_map();
    let name_table = NameTable::gen_name_table();
    find_value_on_map(ip,&mmap, &name_table)
}

pub fn find_value_on_map(ip: u32, mmap: &MmapMut, name_table: &MmapMut) -> Option<String> {
    let node = Tree::find_node_on_map(ip, mmap);
    if node.is_none() { return None }
    let index = node.unwrap();

    if index == 0 { return None }
    let index = index as usize -1; // -1 because we use 0 for tracking if there is no value reference

    NameTable::get_name(&name_table, index)
}
