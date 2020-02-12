use crate::{MemTalker, Node, get_memmap};
use memmap::MmapMut;

pub fn insert_node(mmap: & mut MmapMut, index: usize, node: &Node) {
    MemTalker::place_item(mmap, index, &node);
    if index == 0 { return }
    let root = MemTalker::get_node(&mmap, 0);
    insert_node_on_node(&mmap, root, index, &node);
    //print!("-{}",offset);
}

fn insert_node_on_node(mmap: & MmapMut, parent: &mut Node, index: usize, child: &Node) {

    let mut offset_from_node = 0;

    if parent.min_ip <= child.min_ip && child.max_ip <= parent.max_ip {
        println!("Overlap: {}", std::str::from_utf8(&child.name).expect("Overlap expect"));
        return
    }

    if parent.max_ip < child.max_ip {
        if parent.right == 0 {
            parent.right = index;
            return;
        }
        offset_from_node = parent.right;
    } else if parent.min_ip > child.min_ip {
        if parent.left == 0 {
            parent.left = index;
            return;
        }
        offset_from_node = parent.left;
    }

    let node = MemTalker::get_node(&mmap, offset_from_node);
    insert_node_on_node(mmap, node, index, &child);
}

pub fn find_node_in_tree(ip: u32) -> Option<[u8; 32]> {
    let mmap = get_memmap();
    let mut accNode = MemTalker::get_node(&mmap, 0);

    while true {
        let mut offset_from_node: usize = 0;
        if accNode.min_ip <= ip && ip <= accNode.max_ip { return Some(accNode.name) }

        if accNode.max_ip < ip {
            if accNode.right == 0 { break; }
            offset_from_node = accNode.right;
        } else if accNode.min_ip > ip {
            if accNode.left == 0 { break; }
            offset_from_node = accNode.left;
        }
        accNode = MemTalker::get_node(&mmap, offset_from_node);
    }
    None
}