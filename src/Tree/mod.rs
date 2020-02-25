use core::fmt;
use crate::{Entry, Utils};
use memmap::MmapMut;

pub mod NodeToMem;
mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub struct Node {
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: usize,
    pub right: usize,
    pub name: [u8; 32],
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}, n: {}, min: {}, max: {}, l: {}, r: {}", &self, std::str::from_utf8(&self.name).unwrap(), self.min_ip, self.max_ip, self.left, self.right)
    }
}

pub fn insert_entry(mmap: &mut MmapMut, index: usize, entry: Entry) {
    let node = Utils::entry_to_node(entry);
    Tree::insert_node(mmap, index, &node);
}

pub fn find_value(ip: u32) -> Option<String> {
    let node = Tree::find_node(ip);
    if node.is_none() { return None }
    match std::str::from_utf8(&node.unwrap()) {
        Ok(T) => Some(T.trim_matches(char::from(0)).to_string()),
        Err(E) => None
    }
}
