use crate::{Utils, RedBlack, Table, NameTable, thisFileWillBeDeleted, FileGenerator, SOURCE_PATH_3, SOURCE_PATH_1, REDBLACK_PATH, REDBLACK_PAYLOAD, REDBLACK_PRINT_PATH, SOURCE_PATH_4};
use std::fs::File;
use std::io::{LineWriter, Write, BufRead};
use memmap::MmapMut;
use crate::RedBlack::{Node, NodeToMem};
use crate::RedBlack::Tree::root_index;
use std::fs;
use crate::RedBlack::NodeToMem::get_node;

pub(crate) fn print_tree_to_file(s: &str) {
    fs::remove_file(REDBLACK_PRINT_PATH);
    let file = File::create(s).unwrap();
    let mut line_writer = LineWriter::new(file);
    let tree_map = RedBlack::gen_tree_map();
    let name_table = NameTable::gen_name_table_from_path(REDBLACK_PAYLOAD);
    let root = get_node(&tree_map, unsafe { root_index });
    print_node_to_file(&tree_map, &name_table, &root, 0, &mut line_writer);
}

fn print_node_to_file(mmap: &MmapMut, name_table: &MmapMut, node: &Node, n: usize, writer: &mut LineWriter<File>) {
    if node.right != 0 {
        print_node_to_file(mmap, name_table, &NodeToMem::get_node(&mmap, node.right), n + 1, writer);
    }
    let indention: String = (0..n).map(|_| "---").collect();
    writer.write_all(indention.as_bytes());
    writer.write_all(if node.red { "X " } else { "O " }.as_bytes());
    writer.write_all(NameTable::get_name(&name_table, node.name - 1).unwrap().as_bytes());
    writer.write_all("\n".as_bytes());
    if node.left != 0 {
        print_node_to_file(mmap, name_table, &NodeToMem::get_node(&mmap, node.left), n + 1, writer);
    }
}

#[test]
fn print_tree_and_read_1() {
    let src = thisFileWillBeDeleted;
    fs::remove_file(REDBLACK_PATH);
    crate::load_to_redblack(SOURCE_PATH_1);
    print_tree_to_file(src);
    let mut iter = crate::get_buffer(src).lines().map(|x| x.unwrap() );
    assert_eq!(iter.next(), Some("------X christoffersen".to_string()));
    assert_eq!(iter.next(), Some("---O Pedersen".to_string()));
    assert_eq!(iter.next(), Some("------X Olesen".to_string()));
    assert_eq!(iter.next(), Some("O Hans Hansens Hus".to_string()));
    assert_eq!(iter.next(), Some("---O Siteimprove".to_string()));
    fs::remove_file(src);
    fs::remove_file(REDBLACK_PATH);
}

#[test]
fn print_tree_and_read_2() {
    let src = thisFileWillBeDeleted;
    fs::remove_file(REDBLACK_PATH);
    crate::load_to_redblack(SOURCE_PATH_3);
    print_tree_to_file(src);
    let mut iter = crate::get_buffer(src).lines().map(|x| x.unwrap() );
    assert_eq!(iter.next(), Some("------X christoffersen".to_string()));
    assert_eq!(iter.next(), Some("---O Pedersen".to_string()));
    assert_eq!(iter.next(), Some("------X Olesen".to_string()));
    assert_eq!(iter.next(), Some("O Hans Hansens Hus".to_string()));
    assert_eq!(iter.next(), Some("---O Siteimprove".to_string()));
    fs::remove_file(src);
    fs::remove_file(REDBLACK_PATH);
}

#[test]
#[ignore]
fn print_tree() {
    fs::remove_file(REDBLACK_PATH);
    crate::load_to_redblack(SOURCE_PATH_4);
    print_tree_to_file(REDBLACK_PRINT_PATH);
    fs::remove_file(REDBLACK_PATH);
}