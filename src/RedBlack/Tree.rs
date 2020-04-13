use crate::{NameTable, Entry, RedBlack, REDBLACK_PATH, Utils, usizeSize, thisFileWillBeDeleted, REDBLACK_PAYLOAD};
use memmap::MmapMut;
use crate::RedBlack::{Node, NODE_SIZE, NodeToMem};
use std::ops::Deref;
use crate::RedBlack::TreePrinter;
use std::fs;

pub static mut root_index: usize = 1;

pub fn reset_root_index() { unsafe { root_index = 1 }; }

pub fn save_root_node(map_path: &str) {
    let mut mmap = super::gen_tree_map_on_path(map_path);
    Utils::place_item_raw(&mut mmap, 0, &unsafe { root_index })
}

pub fn load_root_node(map_path: &str) {
    let mut mmap = super::gen_tree_map_on_path(map_path);
    unsafe { root_index = *Utils::bytes_to_type(&mmap[..usizeSize]) }
}

pub fn insert_node(mmap: &mut MmapMut, index: usize, node: &mut Node) {
    NodeToMem::place_node(mmap, index, node);
    mmap.flush();
    if index == 0 {
        panic!("Tried to insert on index 0")
    } else if index != 1 {
        let root = NodeToMem::get_node(mmap, unsafe { root_index });
        insert_node_on_node(mmap, root, 1, node, index);
    }
    balance(mmap, node, index)
}

fn insert_node_on_node(mmap: &MmapMut, parent: &mut Node, parentIndex: usize, node: &mut Node, nodeIndex: usize) {

    let mut offset_from_node = 0;

    if parent.min_ip <= node.min_ip && node.max_ip <= parent.max_ip {
        println!("Overlap for child: {}",node.name);
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
    }

    if node.parent != 0 {
        let mut parent = NodeToMem::get_node(mmap, node.parent);
        //println!("node: {} ----- parent: {}", node,parent);
        if parent.red && parent.parent != 0 {
            let mut grandparent = NodeToMem::get_node(mmap, parent.parent);
            let parentIsLeft = node.parent == grandparent.left;
            let uncleIndex = if parentIsLeft { grandparent.right } else { grandparent.left };
            if uncleIndex != 0 {
                let mut uncle = NodeToMem::get_node(mmap, uncleIndex);
                if uncle.red {
                    //println!("### Uncle");
                    uncle.red = false;
                    parent.red = false;
                    grandparent.red = true;
                    balance(mmap, grandparent, parent.parent);
                    return;
                }
            }
            if node.parent == grandparent.left {
                if parent.left == nodeIndex {
                    //println!("### left left");
                    rightRotate(mmap, parent, grandparent);
                    swapColor(parent,grandparent);
                    //mmap.flush().expect("didnt flush!!");
                } else if parent.right == nodeIndex {
                    //println!("### left right");
                    leftRotate(mmap,node, parent);
                    rightRotate(mmap, node, grandparent);
                    swapColor(node,grandparent);
                } else { panic!("wrong family relation") }
            } else if node.parent == grandparent.right {
                if parent.right == nodeIndex {
                    //println!("### right right");
                    leftRotate(mmap,parent, grandparent);
                    swapColor(parent, grandparent);
                } else if parent.left == nodeIndex {
                    //println!("### right left");
                    rightRotate(mmap,node, parent);
                    leftRotate(mmap,node,grandparent);
                    swapColor(node, grandparent);
                } else { panic!("wrong family relation") }
            }
        }
    }
    let mut mmap2 = super::gen_tree_map();
    NodeToMem::place_node(&mut mmap2, nodeIndex, &node);
}

fn swapColor(node1: & mut Node, node2: &mut Node) {
    let c = node1.red;
    node1.red = node2.red;
    node2.red = c;
}

fn leftRotate(mmap: &MmapMut, node: &mut Node, parent: &mut Node) {
    if node.left != 0 {
        let child = NodeToMem::get_node(mmap, node.left);
        child.parent = node.parent;
    }

    let oldGrandparentIndex = parent.parent;
    parent.parent = parent.right;
    parent.right = node.left;
    node.left = node.parent;
    node.parent = oldGrandparentIndex;

    match oldGrandparentIndex {
        0 => unsafe { root_index = parent.parent },
        _ => {
            let grandparent = NodeToMem::get_node(mmap, oldGrandparentIndex);
            match (grandparent, node) {
                (gp, n) if gp.left == n.left    => gp.left = parent.parent,
                (gp, n) if gp.right == n.left   => gp.right = parent.parent,
                _ => panic!("left rotate: wrong family relation")
            }
        }
    }
}

fn rightRotate(mmap: &MmapMut, node: &mut Node, parent: &mut Node) {
    if node.right != 0 {
        let child = NodeToMem::get_node(mmap, node.right);
        child.parent = node.parent;
    }

    let oldGrandparentIndex = parent.parent;
    parent.parent = parent.left;
    parent.left = node.right;
    node.right = node.parent;
    node.parent = oldGrandparentIndex;

    match oldGrandparentIndex {
        0 => unsafe { root_index = parent.parent },
        _ => {
            let grandparent = NodeToMem::get_node(mmap, oldGrandparentIndex);
            match (grandparent, node) {
                (gp, n) if gp.left == n.right   => gp.left = parent.parent,
                (gp, n) if gp.right == n.right  => gp.right = parent.parent,
                _ => panic!("left rotate: wrong family relation")
            }
        }
    }
}

pub fn find_node_on_map(ip: u32, mmap: &MmapMut) -> Option<usize> {
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

#[test]
fn save_and_load_root_node() {
    unsafe { root_index = 5 }
    save_root_node(thisFileWillBeDeleted);
    unsafe { root_index = 10 }
    assert_eq!(unsafe { root_index }, 10);
    load_root_node(thisFileWillBeDeleted);
    assert_eq!(unsafe { root_index }, 5)
}

#[test]
fn insert_node_and_find_it() {
    RedBlack::reset_root_index();
    fs::remove_file(REDBLACK_PATH);
    fs::remove_file(REDBLACK_PAYLOAD);

    let mut tree_map = super::gen_tree_map();

    let name1 = 10;
    let mut entry = Node { red: true, min_ip: 0, max_ip: 5, left: 0, right: 0, parent: 0, name: name1 };
    insert_node(&mut tree_map, 1, &mut entry);

    let name2 = 20;
    let mut entry = Node { red: true, min_ip: 6, max_ip: 10, left: 0, right: 0, parent: 0, name: name2 };
    insert_node(&mut tree_map, 2, &mut entry);

    let name3 = 30;
    let mut entry = Node { red: true, min_ip: 20, max_ip: 20, left: 0, right: 0, parent: 0, name: name3 };
    insert_node(&mut tree_map, 3, &mut entry);

    let name4 = 40;
    let mut entry = Node { red: true, min_ip: 50, max_ip: 650, left: 0, right: 0, parent: 0, name: name4 };
    insert_node(&mut tree_map, 4, &mut entry);

    let out_name0 = find_node_on_map(0, &tree_map);
    let out_name1 = find_node_on_map(5, &tree_map);
    let out_name2 = find_node_on_map(9, &tree_map);
    let out_name3 = find_node_on_map(20, &tree_map);
    let out_name4 = find_node_on_map(50, &tree_map);
    let out_name5 = find_node_on_map(144, &tree_map);
    let out_name6 = find_node_on_map(650, &tree_map);
    assert!(out_name0.is_some());
    assert!(out_name1.is_some());
    assert!(out_name2.is_some());
    assert!(out_name3.is_some());
    assert!(out_name4.is_some());
    assert!(out_name5.is_some());
    assert!(out_name6.is_some());
    assert_eq!(out_name0.unwrap(),name1);
    assert_eq!(out_name1.unwrap(),name1);
    assert_eq!(out_name2.unwrap(),name2);
    assert_eq!(out_name3.unwrap(),name3);
    assert_eq!(out_name4.unwrap(),name4);
    assert_eq!(out_name5.unwrap(),name4);
    assert_eq!(out_name6.unwrap(),name4);

    let out_name1 = find_node_on_map(40,&tree_map);
    let out_name2 = find_node_on_map(21, &tree_map);
    let out_name3 = find_node_on_map(651, &tree_map);
    assert!(out_name1.is_none());
    assert!(out_name2.is_none());
    assert!(out_name3.is_none());

    fs::remove_file(REDBLACK_PATH);
    fs::remove_file(REDBLACK_PAYLOAD);
}

#[test]
fn insert_node_random_order_and_find_it() {
    unsafe { root_index = 1 };
    fs::remove_file(REDBLACK_PATH);
    fs::remove_file(REDBLACK_PAYLOAD);

    let mut tree_map = super::gen_tree_map();

    let name3 = 30;
    let mut entry = Node { red: true, min_ip: 20, max_ip: 20, left: 0, right: 0, parent: 0, name: name3 };
    insert_node(&mut tree_map, 1, &mut entry);

    let name6 = 40;
    let mut entry = Node { red: true, min_ip: 802, max_ip: 820, left: 0, right: 0, parent: 0, name: name6 };
    insert_node(&mut tree_map, 2, &mut entry);

    let name4 = 40;
    let mut entry = Node { red: true, min_ip: 50, max_ip: 650, left: 0, right: 0, parent: 0, name: name4 };
    insert_node(&mut tree_map, 3, &mut entry);

    let name2 = 20;
    let mut entry = Node { red: true, min_ip: 6, max_ip: 10, left: 0, right: 0, parent: 0, name: name2 };
    insert_node(&mut tree_map, 4, &mut entry);

    let name5 = 40;
    let mut entry = Node { red: true, min_ip: 800, max_ip: 801, left: 0, right: 0, parent: 0, name: name5 };
    insert_node(&mut tree_map, 5, &mut entry);

    let name1 = 10;
    let mut entry = Node { red: true, min_ip: 0, max_ip: 5, left: 0, right: 0, parent: 0, name: name1 };
    insert_node(&mut tree_map, 6, &mut entry);

    let out_name0 = find_node_on_map(0, &tree_map);
    let out_name1 = find_node_on_map(5, &tree_map);
    let out_name2 = find_node_on_map(9, &tree_map);
    let out_name3 = find_node_on_map(20, &tree_map);
    let out_name4 = find_node_on_map(50, &tree_map);
    let out_name5 = find_node_on_map(144, &tree_map);
    let out_name6 = find_node_on_map(650, &tree_map);
    let out_name7 = find_node_on_map(800, &tree_map);
    let out_name8 = find_node_on_map(815, &tree_map);
    assert!(out_name0.is_some());
    assert!(out_name1.is_some());
    assert!(out_name2.is_some());
    assert!(out_name3.is_some());
    assert!(out_name4.is_some());
    assert!(out_name5.is_some());
    assert!(out_name6.is_some());
    assert!(out_name7.is_some());
    assert!(out_name8.is_some());
    assert_eq!(out_name0.unwrap(),name1);
    assert_eq!(out_name1.unwrap(),name1);
    assert_eq!(out_name2.unwrap(),name2);
    assert_eq!(out_name3.unwrap(),name3);
    assert_eq!(out_name4.unwrap(),name4);
    assert_eq!(out_name5.unwrap(),name4);
    assert_eq!(out_name6.unwrap(),name4);
    assert_eq!(out_name7.unwrap(),name5);
    assert_eq!(out_name8.unwrap(),name6);

    fs::remove_file(REDBLACK_PATH);
    fs::remove_file(REDBLACK_PAYLOAD);
}