use crate::{NodeToMem, MAP_PATH};
use memmap::MmapMut;
use crate::Tree::{Node, NODE_SIZE};
use std::ops::Deref;
use crate::Tree::TreePrinter::{print_tree, print_tree_from_map};

pub fn insert_node(mmap: &mut MmapMut, index: usize, node: &mut Node) {
    NodeToMem::place_node(mmap, index, node);
    mmap.flush();
    if index == 0 {
        panic!()
    } else if index != 1 {
        let root = NodeToMem::get_node(mmap, unsafe { root_index });
        insert_node_on_node(mmap, root, 1, node, index);
    }
    balance(mmap, node, index)

    //print!("-{}",offset);
}

fn insert_node_on_node(mmap: &MmapMut, parent: &mut Node, parentIndex: usize, node: &mut Node, nodeIndex: usize) {

    let mut offset_from_node = 0;

    if parent.min_ip <= node.min_ip && node.max_ip <= parent.max_ip {
        println!("Overlap: {}", std::str::from_utf8(&node.name).expect("Overlap expect"));
        return
    }

    if parent.max_ip < node.max_ip {
        if parent.right == 0 {
            parent.right = nodeIndex;
            node.parent = parentIndex;
            let mut mmap2 = super::gen_tree_map();
            NodeToMem::place_node(&mut mmap2, nodeIndex, &node);
            return;
        }
        offset_from_node = parent.right;
    } else if parent.min_ip > node.min_ip {
        if parent.left == 0 {
            parent.left = nodeIndex;
            node.parent = parentIndex;
            let mut mmap2 = super::gen_tree_map();
            NodeToMem::place_node(&mut mmap2, nodeIndex, &node);
            return;
        }
        offset_from_node = parent.left;
    }
    if offset_from_node == 0 { panic!() }

    let childNode = NodeToMem::get_node(mmap, offset_from_node);
    insert_node_on_node(mmap, childNode, offset_from_node, node, nodeIndex);
}

fn balance(mmap: &MmapMut, node: &mut Node, nodeIndex: usize) {
    if nodeIndex == unsafe { root_index } {
        node.red = false;
        //node.max_ip = 2;
        let mut mmap2 = super::gen_tree_map();
        NodeToMem::place_node(&mut mmap2, nodeIndex, &node);

        //println!("Root - Index: {}, Node: {}", nodeIndex, node);
        //println!("root speciel on: \n {:?}",&mmap[NODE_SIZE..NODE_SIZE*5]);
        //mmap.flush();

        print_tree_from_map(&mmap);
        println!();

        return;
    }

    println!("Index: {}, Node: {}", nodeIndex, node);
    println!();
    print_tree_from_map(&mmap);

    if node.parent != 0 {
        let mut parent = NodeToMem::get_node(mmap, node.parent);
        //println!("node: {} ----- parent: {}", node,parent);
        if parent.red {
            if parent.parent == 0 { return; }
            let mut grandparent = NodeToMem::get_node(mmap, parent.parent);
            let parentIsLeft = node.parent == grandparent.left;
            let uncleIndex = if parentIsLeft { grandparent.right } else { grandparent.left };
            if uncleIndex != 0 {
                let mut uncle = NodeToMem::get_node(mmap, uncleIndex);
                if uncle.red {
                    println!("### Uncle");
                    uncle.red = false;
                    parent.red = false;
                    grandparent.red = true;
                    balance(mmap, grandparent, parent.parent);
                    return;
                }
            } else {
                if node.parent == grandparent.left {
                    if parent.left == nodeIndex {
                        println!("### left left");
                        rightRotate(mmap, parent, grandparent);
                        swapColor(parent,grandparent);
                        mmap.flush().expect("didnt flush!!");
                        print_tree_from_map(&mmap);
                        println!();
                    } else if parent.right == nodeIndex {
                        //left right
                        println!("### left right");
                        leftRotate(mmap,node, parent);
                        rightRotate(mmap, node, grandparent);
                        swapColor(node,grandparent);
                        mmap.flush().expect("didnt flush!!");
                        print_tree_from_map(&mmap);
                        println!();
                    } else { panic!() }
                } else if node.parent == grandparent.right {
                    if parent.right == nodeIndex {
                        println!("### right right");
                        leftRotate(mmap,parent, grandparent);
                        swapColor(parent, grandparent);
                        mmap.flush().expect("didnt flush!!");
                        print_tree_from_map(&mmap);
                        println!();
                    } else if parent.left == nodeIndex {
                        println!("### right left");
                        rightRotate(mmap,node, parent);
                        leftRotate(mmap,node,grandparent);
                        swapColor(node, grandparent);
                        mmap.flush().expect("didnt flush!!");
                        print_tree_from_map(&mmap);
                        println!();
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

pub static mut root_index: usize = 1;

fn leftRotate(mmap: &MmapMut, child: &mut Node, parent: &mut Node) {
    let oldGrandparentIndex = parent.parent;
    parent.parent = parent.right;
    parent.right = child.left;
    child.left = child.parent;
    child.parent = oldGrandparentIndex;

    if oldGrandparentIndex == 0 {
        unsafe { root_index = parent.parent };
    } else {
        let grandparent = NodeToMem::get_node(mmap, oldGrandparentIndex);
        if grandparent.left == child.left {
            grandparent.left = parent.parent;
        } else if grandparent.right == child.left {
            grandparent.right = parent.parent;
        } else {
            panic!("wrong family relation")
        }
    }
}

fn rightRotate(mmap: &MmapMut, child: &mut Node, parent: &mut Node) {
    let oldGrandparentIndex = child.parent;
    child.parent = parent.parent;
    parent.parent = parent.left;
    parent.left = child.right;
    child.right = oldGrandparentIndex;

    if child.parent == 0 {
        unsafe { root_index = parent.parent };
        return;
    }
    let grandparent = NodeToMem::get_node(mmap, parent.parent);
    if grandparent.left == child.parent {
        grandparent.left = child.parent;
    } else if grandparent.right == child.parent {
        grandparent.right = child.parent;
    } else {
        panic!("wrong family relation")
    }
}

pub fn find_node_on_map(ip: u32, mmap: &MmapMut) -> Option<[u8; 32]> {
    let mut accNode = NodeToMem::get_node(mmap, unsafe { root_index });

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
        if offset_from_node == 0 { break; }

        accNode = NodeToMem::get_node(&mmap, offset_from_node);
    }
    None
}