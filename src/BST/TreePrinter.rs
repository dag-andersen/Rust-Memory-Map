use crate::{Utils, TREE_PATH, BST, Table, PayloadMap, thisFileWillBeDeleted, FileGenerator, TREE_PRINT_PATH, SOURCE_PATH_3, SOURCE_PATH_1, TREE_PAYLOAD};
use std::fs::File;
use std::io::{LineWriter, Write, BufRead};
use memmap::MmapMut;
use crate::BST::{Node, NodeToMem};
use std::fs;

pub(crate) fn print_tree_to_file(s: &str) {
    fs::remove_file(TREE_PRINT_PATH);
    let file = File::create(s).unwrap();
    let mut line_writer = LineWriter::new(file);
    let tree_map = BST::gen_tree_map();
    let payload_map = PayloadMap::gen_payload_map_from_path(TREE_PAYLOAD);
    let root = NodeToMem::get_node(&tree_map, 0);
    print_node_to_file(&tree_map, &payload_map, &root, 0, &mut line_writer);
}

fn print_node_to_file(mmap: &MmapMut, payload_map: &MmapMut, node: &Node, n: usize, writer: &mut LineWriter<File>) {
    if node.right != 0 {
        print_node_to_file(mmap, payload_map, &NodeToMem::get_node(&mmap, node.right as usize), n + 1, writer);
    }
    let indention: String = (0..n).map(|_| '-').collect();
    writer.write_all(indention.as_bytes());
    writer.write_all(PayloadMap::get_payload(&payload_map, node.payload_ptr as u64 - 1).unwrap().as_bytes());
    writer.write_all("\n".as_bytes());
    if node.left != 0 {
        print_node_to_file(mmap, payload_map, &NodeToMem::get_node(&mmap, node.left as usize), n + 1, writer);
    }
}

#[test]
fn print_tree_and_read_1() {
    let src = thisFileWillBeDeleted;
    fs::remove_file(TREE_PATH);
    crate::load_to_tree(SOURCE_PATH_1);
    print_tree_to_file(src);
    let mut iter = crate::get_buffer(src).lines().map(|x| x.unwrap() );
    assert_eq!(iter.next(), Some("----christoffersen".to_string()));
    assert_eq!(iter.next(), Some("---Pedersen".to_string()));
    assert_eq!(iter.next(), Some("--Olesen".to_string()));
    assert_eq!(iter.next(), Some("-Hans Hansens Hus".to_string()));
    assert_eq!(iter.next(), Some("Siteimprove".to_string()));
    fs::remove_file(src);
    fs::remove_file(TREE_PATH);
}

#[test]
fn print_tree_and_read_2() {
    let src = thisFileWillBeDeleted;
    fs::remove_file(TREE_PATH);
    crate::load_to_tree(SOURCE_PATH_3);
    print_tree_to_file(src);
    let mut iter = crate::get_buffer(src).lines().map(|x| x.unwrap() );
    assert_eq!(iter.next(), Some("--christoffersen".to_string()));
    assert_eq!(iter.next(), Some("---Pedersen".to_string()));
    assert_eq!(iter.next(), Some("-Olesen".to_string()));
    assert_eq!(iter.next(), Some("--Hans Hansens Hus".to_string()));
    assert_eq!(iter.next(), Some("Siteimprove".to_string()));
    fs::remove_file(src);
    fs::remove_file(TREE_PATH);
}