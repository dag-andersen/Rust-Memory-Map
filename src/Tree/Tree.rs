use crate::{NodeToMem, TREE_PATH, NameTable, TREE_MAP_500_000, NAME_TABLE, Entry};
use memmap::MmapMut;
use crate::Tree::Node;
use std::fs;

pub fn insert_node(mmap: & mut MmapMut, index: usize, node: &Node) {
    NodeToMem::place_node(mmap, index, &node);
    if index == 0 { return }
    let root = NodeToMem::get_node(&mmap, 0);
    insert_node_on_node(&mmap, root, index, &node);
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


#[test]
fn insert_node_and_find_it() {
    fs::remove_file(TREE_PATH);
    fs::remove_file(NAME_TABLE);

    let mut tree_map = super::gen_tree_map();

    let name1 = 10;
    let entry = Node { min_ip: 0, max_ip: 5, left: 0, right: 0, name: name1 };
    insert_node(&mut tree_map, 0, &entry);

    let name2 = 20;
    let entry = Node { min_ip: 6, max_ip: 10, left: 0, right: 0, name: name2 };
    insert_node(&mut tree_map, 1, &entry);

    let name3 = 30;
    let entry = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name3 };
    insert_node(&mut tree_map, 2, &entry);

    let name4 = 40;
    let entry = Node { min_ip: 50, max_ip: 650, left: 0, right: 0, name: name4 };
    insert_node(&mut tree_map, 3, &entry);

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

    fs::remove_file(TREE_PATH);
    fs::remove_file(NAME_TABLE);
}

#[test]
fn insert_node_random_order_and_find_it() {
    fs::remove_file(TREE_PATH);
    fs::remove_file(NAME_TABLE);

    let mut tree_map = super::gen_tree_map();

    let name3 = 30;
    let entry = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name3 };
    insert_node(&mut tree_map, 0, &entry);

    let name6 = 40;
    let entry = Node { min_ip: 802, max_ip: 820, left: 0, right: 0, name: name6 };
    insert_node(&mut tree_map, 1, &entry);

    let name4 = 40;
    let entry = Node { min_ip: 50, max_ip: 650, left: 0, right: 0, name: name4 };
    insert_node(&mut tree_map, 2, &entry);

    let name2 = 20;
    let entry = Node { min_ip: 6, max_ip: 10, left: 0, right: 0, name: name2 };
    insert_node(&mut tree_map, 3, &entry);

    let name5 = 40;
    let entry = Node { min_ip: 800, max_ip: 801, left: 0, right: 0, name: name5 };
    insert_node(&mut tree_map, 4, &entry);

    let name1 = 10;
    let entry = Node { min_ip: 0, max_ip: 5, left: 0, right: 0, name: name1 };
    insert_node(&mut tree_map, 5, &entry);

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

    fs::remove_file(TREE_PATH);
    fs::remove_file(NAME_TABLE);
}