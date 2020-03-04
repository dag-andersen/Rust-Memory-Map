use crate::{NodeToMem, MAP_PATH};
use memmap::MmapMut;
use crate::Tree::Node;

pub fn insert_node(mmap: &mut MmapMut, index: usize, node: &mut Node) {
    NodeToMem::place_node(mmap, index, &node);
    if index == 0 { return }
    let root = NodeToMem::get_node(mmap, 0);
    insert_node_on_node(mmap, root, index, node);
    //print!("-{}",offset);
}

fn insert_node_on_node(mmap: &MmapMut, parent: &mut Node, index: usize, node: &mut Node) {

    let mut offset_from_node = 0;

    if parent.min_ip <= node.min_ip && node.max_ip <= parent.max_ip {
        println!("Overlap: {}", std::str::from_utf8(&node.name).expect("Overlap expect"));
        return
    }

    if parent.max_ip < node.max_ip {
        if parent.right == 0 {
            parent.right = index;
            return;
        }
        offset_from_node = parent.right;
    } else if parent.min_ip > node.min_ip {
        if parent.left == 0 {
            parent.left = index;
            return;
        }
        offset_from_node = parent.left;
    }

    let childNode = NodeToMem::get_node(mmap, offset_from_node);
    insert_node_on_node(mmap, childNode, index, node);
}

fn balance(mmap: &MmapMut, node: &mut Node, nodeIndex: usize) {
    let mut parent = NodeToMem::get_node(mmap, node.parent);
    let mut grandparent = NodeToMem::get_node(mmap, parent.parent);
    let parentIsLeft = node.parent == grandparent.left;
    let mut uncle = NodeToMem::get_node(mmap, if parentIsLeft { grandparent.right } else { grandparent.left });

    if node.red {
        if parent.red {
            if uncle.red {
                uncle.red = false;
                parent.red = false;
                grandparent.red = true;
                balance(mmap, grandparent, parent.parent);
                return;
            } else {
                let grandgrandparent = NodeToMem::get_node(mmap, grandparent.parent);
                if node.parent == grandparent.left {
                    if parent.left == nodeIndex {
                        //left left
                        rightRotate(parent, grandparent, grandgrandparent);
                        swapColor(parent,grandparent);
                    } else if parent.right == nodeIndex {
                        //left right
                        leftRotate(node,parent,grandparent);
                        rightRotate(node, grandparent, grandgrandparent);
                        swapColor(node,grandparent);
                    } else { panic!() }
                } else if node.parent == grandparent.right {
                    if parent.right == nodeIndex {
                        //right right
                        leftRotate(parent,grandparent, grandgrandparent);
                        swapColor(parent, grandparent);
                    } else if parent.left == nodeIndex {
                        //right left
                        rightRotate(node, parent, grandgrandparent);
                        leftRotate(node,grandparent, grandgrandparent);
                        swapColor(node, grandparent);
                    }
                }
            }
        }
    }
}

fn swapColor(node1: & mut Node, node2: &mut Node) {
    node1.red = node2.red;
    node2.red = !node1.red;
}

fn leftRotate(child: &mut Node, parent: &mut Node, grandparent: &mut Node) {
    let oldGrandparentIndex = parent.parent;
    grandparent.left = parent.right;
    parent.right = child.left;
    child.left = child.parent;
    child.parent = oldGrandparentIndex;
    parent.parent = grandparent.left;
}

fn rightRotate(child: &mut Node, parent: &mut Node, grandparent: &mut Node) {
    let oldGrandparentIndex = child.parent;
    child.parent = parent.parent;
    parent.parent = parent.left;
    parent.left = child.right;
    child.right = oldGrandparentIndex;
    if grandparent.left == child.parent {
        grandparent.left = child.parent;
    } else if grandparent.right == child.parent {
        grandparent.right = child.parent;
    } else {
        panic!("wrong family relation")
    }
}

pub fn find_node_on_map(ip: u32, mmap: &MmapMut) -> Option<[u8; 32]> {
    let mut accNode = NodeToMem::get_node(mmap, 0);

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