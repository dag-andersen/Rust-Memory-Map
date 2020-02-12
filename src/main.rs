#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const SOURCE_PATH_1:    &str = "testdata/in/set1.txt";
const SOURCE_PATH_2:    &str = "testdata/in/set2.txt";
const SOURCE_PATH_3:    &str = "testdata/in/set3.txt";
const MAP_PATH:         &str = "testdata/out/map.txt";
const TREE_PRINT_PATH:  &str = "testdata/out/tree.txt";
const NODE_SIZE : usize = std::mem::size_of::<Node>();

mod Tree;
mod FileGenerator;
mod TreePrinter;
mod MemTalker;

use std::io::{BufRead, BufReader, LineWriter};
use std::ops::Add;
use memmap::{MmapMut, MmapOptions};
use std::io::Read;
use std::{fs::{OpenOptions, File}, io::{Seek, SeekFrom, Write}, os::unix::prelude::AsRawFd, ptr, fs, mem, fmt};
use regex::bytes::Regex;
use std::cmp::min;
use rand::{Rng, random};
use std::io::prelude::*;
use rand::distributions::Alphanumeric;
use rand::prelude::ThreadRng;

pub struct Node {
    min_ip: u32,
    max_ip: u32,
    left: usize,
    right: usize,
    name: [u8; 32],
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:p}, n: {}, min: {}, max: {}, l: {}, r: {}", &self, std::str::from_utf8(&self.name).unwrap(), self.min_ip, self.max_ip, self.left, self.right)
    }
}

fn insert_array_in_array(one: & mut [u8; 32], two: &[u8])  {
    for (place, data) in one.iter_mut().zip(two.iter()) {
        *place = *data
    }
}

fn main() {
    store_scr_on_map(SOURCE_PATH_1);
}

fn store_scr_on_map(scr: &str) {
    fs::remove_file(MAP_PATH);

    let mut mmap = get_memmap();

    let ip_regex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
    let name_regex = Regex::new(r"\b(([A-z]|\d)+\s?)+\b").unwrap();

    let buffer = get_buffer(scr);

    for (i, line) in buffer.lines().enumerate() {
        if line.is_err() { continue }
        let l = line.unwrap();
        if l.is_empty() { continue; }

        let node = get_node_for_line(&ip_regex,&name_regex,l);

        if node.is_none() { continue }
        let node = node.unwrap();

        Tree::insert_node(& mut mmap,i, &node);
    }
}

fn get_node_for_line(ip_regex: &Regex, name_regex: &Regex, l: String) -> Option<Node> {

    let min_ip_match = ip_regex.find(l.as_bytes()).expect("didnt find min ip");
    let max_ip_match = ip_regex.find_at(l.as_bytes(), min_ip_match.end()).expect("didnt find max ip");
    let name_match = name_regex.find_at(l.as_bytes(), max_ip_match.end()).expect("didnt find name");

    //println!("min:{}- max:{}- name:{}", &l[min_ip_match.range()], &l[max_ip_match.range()], &l[name_match.range()]);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, name_match.as_bytes());

    let min_ip = get_u32_for_ip(&l[min_ip_match.range()])?;
    let max_ip = get_u32_for_ip(&l[max_ip_match.range()])?;

    Some(Node { min_ip, max_ip, left: 0, right: 0, name, })
}

fn get_u32_for_ip(v: &str ) -> Option<u32> {
    let v: Vec<&str> = v.split('.').collect();
    if v.len() != 4 { return None }
    let mut min_array: [u8; 4] = Default::default();
    for i in 0..v.len() {
        min_array[i] = match v[i].parse() {
            Ok(n) => n,
            Err(e) => return None
        }
    }
    //println!("IP?{}.{}.{}.{}",min_array[0],min_array[1],min_array[2],min_array[3]);
    Some(u32::from_be_bytes(min_array))
}

fn get_buffer(file: &str) -> BufReader<std::fs::File> {
    BufReader::new(File::open(file).expect("could not find file"))
}

fn get_memmap() -> MmapMut {
    const SIZE: u64 = 128 * 128 * 128 * 128;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(MAP_PATH)
        .expect("Unable to open file");
    file.seek(SeekFrom::Start(SIZE)).unwrap();
    file.write_all(&[0]).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut mmap = unsafe { MmapOptions::new().map_mut( & file).unwrap() };
    mmap
}

#[test]
fn test_print_tree_to_file() {
    let src = "thisFileWillBeDeleted";
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    store_scr_on_map(src);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}

#[test]
fn test_find_random() {
    let src = "test_find_random";
    let numberOfLines = 100;
    FileGenerator::generate_source_file(numberOfLines, src);
    store_scr_on_map(src);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "testname".as_bytes());

    let ip = 34568;

    let node = Node { min_ip: ip-1, max_ip: ip+1, left: 0, right: 0, name, };

    let mut mmap = get_memmap();
    Tree::insert_node(&mut mmap,numberOfLines, &node);

    let getNode = Tree::find_node_in_tree(ip);
    assert!(getNode.is_some());
    let getNode = getNode.unwrap();
    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getNode).unwrap();
    assert_eq!(left, right);

    fs::remove_file(src);
}

#[test]
fn test_get_ip_for_line() {
    let ip_str = "0.0.0.132";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_some());
    assert_eq!(ip_u32.unwrap(),132);

    let ip_str = "0.0.1.1";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_some());
    assert_eq!(ip_u32.unwrap(),257);

    let ip_str = "0.0.0.300";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_none());

    let ip_str = "0.1.1";
    let ip_u32 = get_u32_for_ip(&ip_str);
    assert!(ip_u32.is_none());
}

#[test]
fn test_find_node_in_tree() {
    store_scr_on_map(SOURCE_PATH_1);

    let name = Tree::find_node_in_tree(get_u32_for_ip("000.000.000.015").unwrap());
    assert!(name.is_some());
    let name = name.unwrap();
    let strName = std::str::from_utf8(&name).unwrap().trim_matches(char::from(0));
    assert_eq!(strName,"Siteimprove");

    let name = Tree::find_node_in_tree(get_u32_for_ip("000.000.002.015").unwrap());
    assert!(name.is_some());
    let name = name.unwrap();
    let strName = std::str::from_utf8(&name).unwrap().trim_matches(char::from(0));
    assert_eq!(strName,"Olesen");

    let name = Tree::find_node_in_tree(get_u32_for_ip("000.000.000.001").unwrap());
    assert!(name.is_none());

    let name = Tree::find_node_in_tree(get_u32_for_ip("001.000.000.000").unwrap());
    assert!(name.is_none());
}


#[test]
fn test_correct_placement() {
    fs::remove_file(MAP_PATH);
    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node1 = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: Default::default(), };
    let node2 = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name, };

    let mut first_map = get_memmap();
    MemTalker::place_item(& mut first_map, 0, &node1);
    MemTalker::place_item(& mut first_map, 1, &node2);

    let mut another_map = get_memmap();
    let getnode = MemTalker::get_node(&another_map, 1);

    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getnode.name).unwrap();
    assert_eq!(left, right);
}




//(\d{1,3}[.]){3}(\d{1,3})|(\w+\s?)+
//(\d{1,3}[.]){3}(\d{1,3})\s
//let minNumber = u32::from_be_bytes(a);

//let test = std::str::from_utf8(&a).unwrap();
//println!("crazy test: {}",test);

//let len = cmp::min(a.len(), asdf);
//bytes::copy_memory(a.mut_slice_to(len), &name.as_bytes()[0..4].slice_to(len));

//let strrr = String::from(l);
//strrr.s

//a.copy_from_slice(&name.as_bytes()[..(min(10,length-1))]);
//let minNumber = u32::from_be_bytes(a);
//println!("test:{}", minNumber);

//let test = std::str::from_utf8(&a).unwrap();
//println!("crazy test: {}",test);