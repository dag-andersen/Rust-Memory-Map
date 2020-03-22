use crate::{NodeToMem, MAP_PATH};
use memmap::MmapMut;
use crate::Tree::Node;

pub fn insert_node(mmap: & mut MmapMut, index: usize, node: &Node) {
    NodeToMem::place_node(mmap, index, &node);
    if index == 0 { return }
    let root = NodeToMem::get_node(&mmap, 0);
    insert_node_on_node(&mmap, root, index, &node);
    //print!("-{}",offset);
}

fn insert_node_on_node(mmap: & MmapMut, parent: &mut Node, index: usize, child: &Node) {

    let mut offset_from_node = 0;

    if parent.min_ip <= child.min_ip && child.max_ip <= parent.max_ip {
        println!("Overlap for child: {}",child.name);
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

    let node = NodeToMem::get_node(&mmap, offset_from_node);
    insert_node_on_node(mmap, node, index, &child);
}

pub fn find_node_on_map(ip: u32, mmap: &MmapMut) -> Option<usize> {
    let mut accNode = NodeToMem::get_node(&mmap, 0);

    loop {
        let mut offset_from_node: usize = 0;
        if accNode.min_ip <= ip && ip <= accNode.max_ip { return Some(accNode.name) }

        if accNode.max_ip < ip {
            if accNode.right == 0 { break; }
            offset_from_node = accNode.right;
        } else if accNode.min_ip > ip {
            if accNode.left == 0 { break; }
            offset_from_node = accNode.left;
        }
        accNode = NodeToMem::get_node(&mmap, offset_from_node);
    }
    None
}