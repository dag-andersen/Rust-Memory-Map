use crate::{Utils, RedBlack, Table, NameTable, thisFileWillBeDeleted, FileGenerator, TREE_PRINT_PATH, SOURCE_PATH_3, SOURCE_PATH_1, REDBLACK_PATH};
use std::fs::File;
use std::io::{LineWriter, Write, BufRead};
use memmap::MmapMut;
use crate::RedBlack::{Node, NodeToMem};
use crate::RedBlack::Tree::root_index;
use std::fs;
use crate::RedBlack::NodeToMem::get_node;

pub(crate) fn print_tree() {
    let mmap = Utils::get_memmap(REDBLACK_PATH, 3000000);
    let root = get_node(&mmap, unsafe { root_index });
    print_node(&mmap, &root, 0)
}

pub(crate) fn print_tree_from_map(mmap: &MmapMut) {
    let root = get_node(&mmap, unsafe { root_index });
    print_node(&mmap, &root, 0)
}

fn print_node(mmap: &MmapMut, node: &Node, n: usize) {
    if node.right != 0 {
        print_node(mmap, &NodeToMem::get_node(&mmap, node.right), n + 1);
    }
    let indention : String = (0..n).map(|_| '-').collect();
    print!("{}",indention);
    print!("{}", if node.red { "X - red - " } else { "O - black - " });
    println!("{}", &node.name);
    if node.left != 0 {
        print_node(mmap, &NodeToMem::get_node(&mmap, node.left), n + 1);
    }
}

pub(crate) fn print_tree_to_file(s: &str) {
    fs::remove_file(TREE_PRINT_PATH);
    let file = File::create(s).unwrap();
    let mut line_writer = LineWriter::new(file);
    let tree_map = RedBlack::gen_tree_map();
    let name_table = NameTable::gen_name_table();
    let root = get_node(&tree_map, unsafe { root_index });
    print_node_to_file(&tree_map, &name_table, &root, 0, &mut line_writer);
}

fn print_node_to_file(mmap: &MmapMut, lookup: &MmapMut, node: &Node, n: usize, writer: &mut LineWriter<File>) {
    if node.right != 0 {
        print_node_to_file(mmap, lookup, &NodeToMem::get_node(&mmap, node.right), n + 1, writer);
    }
    let indention: String = (0..n).map(|_| "---").collect();
    writer.write_all(indention.as_bytes());
    writer.write_all(if node.red { "X " } else { "O " }.as_bytes());
    writer.write_all(NameTable::get_name(&lookup, node.name - 1).unwrap().as_bytes());
    writer.write_all("\n".as_bytes());
    if node.left != 0 {
        print_node_to_file(mmap, lookup, &NodeToMem::get_node(&mmap, node.left), n + 1, writer);
    }
}

#[test]
fn print_tree_and_read() {
    RedBlack::reset_root_index();
    let src = thisFileWillBeDeleted;
    fs::remove_file(REDBLACK_PATH);
    crate::load_to_redblack(SOURCE_PATH_1);
    RedBlack::TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    let mut iter = crate::get_buffer(TREE_PRINT_PATH).lines().map(|x| x.unwrap() );
    assert_eq!(iter.next(), Some("------X christoffersen".to_string()));
    assert_eq!(iter.next(), Some("---O Pedersen".to_string()));
    assert_eq!(iter.next(), Some("------X Olesen".to_string()));
    assert_eq!(iter.next(), Some("O Hans Hansens Hus".to_string()));
    assert_eq!(iter.next(), Some("---O Siteimprove".to_string()));
    fs::remove_file(TREE_PRINT_PATH);
    fs::remove_file(src);
    fs::remove_file(REDBLACK_PATH);
}

#[test]
fn print_tree_and_read_2() {
    RedBlack::reset_root_index();
    let src = thisFileWillBeDeleted;
    fs::remove_file(REDBLACK_PATH);
    crate::load_to_redblack(SOURCE_PATH_3);
    RedBlack::TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    let mut iter = crate::get_buffer(TREE_PRINT_PATH).lines().map(|x| x.unwrap() );
    assert_eq!(iter.next(), Some("------X christoffersen".to_string()));
    assert_eq!(iter.next(), Some("---O Pedersen".to_string()));
    assert_eq!(iter.next(), Some("------X Olesen".to_string()));
    assert_eq!(iter.next(), Some("O Hans Hansens Hus".to_string()));
    assert_eq!(iter.next(), Some("---O Siteimprove".to_string()));
    fs::remove_file(TREE_PRINT_PATH);
    fs::remove_file(src);
    fs::remove_file(REDBLACK_PATH);
}