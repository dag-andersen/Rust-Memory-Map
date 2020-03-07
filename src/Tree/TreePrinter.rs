use crate::{Utils, MAP_PATH, Tree};
use std::fs::File;
use std::io::{LineWriter, Write};
use memmap::MmapMut;
use crate::NodeToMem::get_node;
use crate::Tree::Node;
use crate::Tree::Tree::root_index;

pub(crate) fn print_tree() {
    let mmap = Utils::get_memmap(MAP_PATH, 3000000);
    let root = get_node(&mmap, unsafe { root_index });
    print_node(&mmap, &root, 0)
}

pub(crate) fn print_tree_from_map(mmap: &MmapMut) {
    let root = get_node(&mmap, unsafe { root_index });
    print_node(&mmap, &root, 0)
}

fn print_node(mmap: &MmapMut, node: &Node, n: usize) {
    if node.right != 0 {
        print_node(mmap, &get_node(&mmap, node.right), n + 1);
    }
    let indention : String = (0..n).map(|_| '-').collect();
    print!("{}",indention);
    print!("{}", if node.red { "red - " } else { "black - " });
    println!("{}", std::str::from_utf8(&node.name).unwrap());
    if node.left != 0 {
        print_node(mmap, &get_node(&mmap, node.left), n + 1);
    }
}

pub(crate) fn print_tree_to_file(s: &str) {
    let file = File::create(s).unwrap();
    let mut file = LineWriter::new(file);
    let mmap = Tree::gen_tree_map();
    let root = get_node(&mmap, unsafe { root_index });
    print_node_to_file(&mmap, &root, 0, &mut file);
}

fn print_node_to_file(mmap: &MmapMut, node: &Node, n: usize, writer: &mut LineWriter<File>) {
    if node.right != 0 {
        print_node_to_file(mmap, &get_node(&mmap, node.right), n + 1, writer);
    }
    let indention : String = (0..n).map(|_| '-').collect();
    writer.write_all(indention.as_bytes());
    writer.write_all(&node.name);
    writer.write_all(if node.red { "- red" } else { "- black" }.as_bytes());
    writer.write_all("\n".as_bytes());
    if node.left != 0 {
        print_node_to_file(mmap, &get_node(&mmap, node.left), n + 1, writer);
    }
}