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

const MAP_PATH: &str = "testdata/db.txt";
const SOURCE_PATH_1: &str = "testdata/set1.txt";
const SOURCE_PATH_2: &str = "testdata/set2.txt";
const SOURCE_PATH_3: &str = "testdata/set3.txt";
const NODE_SIZE : usize = std::mem::size_of::<Node>();

use std::io::{BufRead, BufReader, LineWriter};
use std::ops::Add;
use memmap::{MmapMut, MmapOptions};
use std::io::Read;
use std::{fs::{OpenOptions, File}, io::{Seek, SeekFrom, Write}, os::unix::prelude::AsRawFd, ptr, fs, mem, fmt};
use bytes::{MutBuf, ToBytes};
use regex::bytes::Regex;
use std::cmp::min;
use rand::{Rng, random};
use std::io::prelude::*;
use rand::distributions::Alphanumeric;

struct Node {
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

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

fn node_from_bytes(slice: &[u8]) -> &mut Node { unsafe { bytes_to_typed(slice) } }

pub unsafe fn bytes_to_typed<T>(slice: &[u8]) -> &mut T {
    std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, std::mem::size_of::<T>())
        .get_mut(0)
        .unwrap()
}

fn node_to_bytes(node: &Node) -> &[u8] { unsafe { any_as_u8_slice(node) } }

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

    let buffer = get_buffer(scr);

    for (i, line) in buffer.lines().enumerate() {
        if line.is_err() { continue }
        let l = line.unwrap();
        if l.is_empty() { continue; }

        let node = get_node_for_line(l);

        if node.is_none() { continue }
        let node = node.unwrap();

        insert_node(& mut mmap,i, &node);
    }
}

fn get_node_for_line(l: String) -> Option<Node> {
    let ip_regex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
    let name_regex = Regex::new(r"\b(([A-z]|\d)+\s?)+\b").unwrap();
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
    if v.len() < 4 { return None }
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

fn insert_node(mmap: & mut MmapMut, offset: usize, node: &Node) {
    place_item(mmap, offset, &node);
    if offset == 0 { return }
    let root = get_node(&mmap, 0);
    insert_node_on_node(&mmap, root, offset, &node);
    print!("-{}",offset);
}

fn insert_node_on_node(mmap: & MmapMut, parent: &mut Node, offset: usize, child: &Node) {

    let mut offset_from_node = 0;

    if parent.min_ip <= child.min_ip && child.max_ip <= parent.max_ip {
        println!("Overlap: {}", std::str::from_utf8(&child.name).unwrap());
        return
    }

    if parent.max_ip < child.max_ip {
        if parent.right == 0 {
            parent.right = offset;
            return;
        }
        offset_from_node = parent.right;
    } else if parent.min_ip > child.min_ip {
        if parent.left == 0 {
            parent.left = offset;
            return;
        }
        offset_from_node = parent.left;
    }

    let node= get_node(&mmap, offset_from_node);
    insert_node_on_node(mmap, node, offset, &child);
}

fn get_node<'a>(mmap: &'a MmapMut, index: usize) -> &'a mut Node {
    get_node_raw(mmap,index*NODE_SIZE)
}

fn get_node_raw<'a>(mmap: &'a MmapMut, offset: usize) -> &'a mut Node {
    let byte_map = &mmap[offset..(offset+NODE_SIZE)];
    let node = node_from_bytes(&byte_map);
    node
}

fn find_node_in_tree(ip: u32) -> Option<[u8; 32]> {
    let mmap = get_memmap();
    let mut accNode = get_node(&mmap, 0);

    while true {
        let mut offset_from_node: usize = 0;
        if accNode.min_ip <= ip && ip <= accNode.max_ip { return Some(accNode.name) }

        if accNode.max_ip < ip {
            if accNode.right == 0 {
                break;
            }
            offset_from_node = accNode.right;
        } else if accNode.min_ip > ip {
            if accNode.left == 0 {
                break;
            }
            offset_from_node = accNode.left;
        }
        accNode = get_node(&mmap, offset_from_node);
    }
    None
}

fn find_node(ip: u32) -> Option<[u8; 32]> {
    let mut counter: usize = 0;
    let mut mmap = get_memmap();
    while counter < 50 {
        let node = get_node(&mmap, counter);
        if node.min_ip < ip && ip < node.max_ip { return Some(node.name) }
        counter += 1;
    }
    None
}

fn place_item(mmap: & mut MmapMut, index: usize, node: & Node) {
    place_item_raw(mmap,index * NODE_SIZE,node);
}

fn place_item_raw(mmap: & mut MmapMut, offset: usize, node: & Node,) {
    let bytes = node_to_bytes(node);
    mmap[offset..(offset+bytes.len())].copy_from_slice(bytes);
    //mmap.flush();
    //mmap.flush_range(offset, bytes.len());
    //println!("{:p}",&mmap[offset]);
    //println!("{}", node)
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

fn print_tree() {
    let mmap = get_memmap();
    let root = get_node(&mmap, 0);
    print_node(&mmap, &root, 0)
}

fn print_node(mmap: &MmapMut, node: &Node, n: usize) {
    let indention : String = (0..n).map(|_| '-').collect();
    if node.right != 0 {
        let rightNode = get_node(&mmap, node.right);
        print_node(mmap, &rightNode, n + 1);
    }
    print!("{}",indention);
    println!("{}", std::str::from_utf8(&node.name).unwrap());
    if node.left != 0 {
        let leftNode = get_node(&mmap, node.left);
        print_node(mmap, &leftNode, n + 1);
    }
}

#[test]
fn test_print_tree() {
    let src = "thisFileWillBeDeleted";
    generate_source_file(200, src);
    store_scr_on_map(src);
    print_tree();
    fs::remove_file(src);
}

#[test]
fn test_make_40_file() {
    let src = "40gb";
    generate_source_file(1000, src);
    store_scr_on_map(src);
    print_tree();
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
    let mut mmap = get_memmap();

    let name = find_node_in_tree(get_u32_for_ip("000.000.000.015").unwrap());
    assert!(name.is_some());
    let name = name.unwrap();
    let strName = std::str::from_utf8(&name).unwrap().trim_matches(char::from(0));
    assert_eq!(strName,"Siteimprove");

    let name = find_node_in_tree(get_u32_for_ip("000.000.002.015").unwrap());
    assert!(name.is_some());
    let name = name.unwrap();
    let strName = std::str::from_utf8(&name).unwrap().trim_matches(char::from(0));
    assert_eq!(strName,"Olesen");

    let name = find_node_in_tree(get_u32_for_ip("000.000.000.001").unwrap());
    assert!(name.is_none());

    let name = find_node_in_tree(get_u32_for_ip("001.000.000.000").unwrap());
    assert!(name.is_none());
}

#[test]
fn test_find() {
    fs::remove_file(MAP_PATH);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let min_ip = 20;
    let max_ip = 40;

    let node1 = Node { min_ip, max_ip: 0, left: 0, right: 0, name: Default::default(), };
    let node2 = Node { min_ip, max_ip: 40, left: 0, right: 0, name, };
    let node3 = Node { min_ip, max_ip: 0, left: 0, right: 0, name: Default::default(), };

    let mut first_map = get_memmap();
    place_item(& mut first_map, 0, &node1);
    place_item(& mut first_map, 1, &node2);
    place_item(& mut first_map, 2, &node3);

    let mut another_map = get_memmap();
    let temp = find_node((min_ip+max_ip)/2);
    assert!(temp.is_some());
    let gotten_name = find_node((min_ip+max_ip)/2).unwrap();

    assert_eq!(gotten_name, name);
}

#[test]
fn test_place_item_and_get() {
    fs::remove_file(MAP_PATH);
    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name, };

    let mut first_map = get_memmap();
    place_item(& mut first_map, 0, &node);

    let mut another_map = get_memmap();
    let getnode = get_node(&another_map, 0);

    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getnode.name).unwrap();

    assert_eq!(left, right);
}

#[test]
fn test_correct_placement() {
    fs::remove_file(MAP_PATH);
    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node1 = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: Default::default(), };
    let node2 = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name, };

    let mut first_map = get_memmap();
    place_item(& mut first_map, 0, &node1);
    place_item(& mut first_map, 1, &node2);

    let mut another_map = get_memmap();
    let getnode = get_node(&another_map, 1);

    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getnode.name).unwrap();
    assert_eq!(left, right);
}

#[test]
#[should_panic]
fn test_correct_placement_panic() {
    fs::remove_file(MAP_PATH);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name, };

    let mut first_map = get_memmap();
    place_item(& mut first_map, 0, &node);

    let mut another_map = get_memmap();
    //get something that doesnt exists
    let getnode = get_node(&another_map, 1);

    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getnode.name).unwrap();
    assert_eq!(left, right);
}

fn generate_random_ip_firm() -> String {
    let mut rng = rand::thread_rng();
    let ip1 : u8 = rng.gen();
    let ip2 : u8 = rng.gen();
    let ip3 : u8 = rng.gen();
    let ip4 : u8 = rng.gen();
    let mut r = String::new();
    r.push_str(&format!("{}",ip1)); r.push('.');
    r.push_str(&format!("{}",ip2)); r.push('.');
    r.push_str(&format!("{}",ip3)); r.push('.');
    r.push_str(&format!("{}",ip4 >> 1)); r.push(' ');
    r.push_str(&format!("{}",ip1)); r.push('.');
    r.push_str(&format!("{}",ip2)); r.push('.');
    r.push_str(&format!("{}",ip3)); r.push('.');
    r.push_str(&format!("{}",ip4)); r.push(' ');
    let name = rng.sample_iter(&Alphanumeric).take(10).collect::<String>();
    r.push_str(&name); r.push_str("\n");
    r
}

fn generate_source_file(n: usize, s:&str) {
    let file = File::create(s).unwrap();
    let mut file = LineWriter::new(file);
    for i in 0..n {
        if i % 100 == 0 { println!("number of lines created: {}", i); }
        let s = generate_random_ip_firm();
        file.write_all( s.as_bytes());
        file.flush();
        //print!("{}", s);
    }
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