use core::fmt;
use crate::{Entry, Utils, MAP_PATH};
use memmap::MmapMut;

pub mod NodeToMem;
mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub struct Node {
    pub red: bool,
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: usize,
    pub right: usize,
    pub parent: usize,
    pub name: [u8; 32],
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "red: {}, n: {}, min: {}, max: {}, l: {}, r: {}, p: {}", self.red, std::str::from_utf8(&self.name).unwrap(), self.min_ip, self.max_ip, self.left, self.right, self.parent)
    }
}

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry) {
    let mut node = Utils::entry_to_node(entry);
    Tree::insert_node(mmap, index, &mut node);
}

pub fn gen_tree_map() -> MmapMut { gen_tree_map_on_path(MAP_PATH) }
pub fn gen_tree_map_on_path(path: &str) -> MmapMut { Utils::get_memmap(path, 20_000_000_000) }

pub fn find_value(ip: u32) -> Option<String> {
    let mmap = gen_tree_map();
    find_value_on_map(ip,&mmap)
}

pub fn find_value_on_map(ip: u32, mmap: &MmapMut) -> Option<String> {
    let node = Tree::find_node_on_map(ip, mmap);
    if node.is_none() { return None }
    match std::str::from_utf8(&node.unwrap()) {
        Ok(T) => Some(T.trim_matches(char::from(0)).to_string()),
        Err(E) => None
    }
}
