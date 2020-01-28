const MAP_PATH: &str = "testdata/db.txt";
const SOURCE_PATH: &str = "testdata/set1.txt";
const NODE_SIZE : usize = std::mem::size_of::<Node>();

use std;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::collections::HashMap;
use memmap::{MmapMut, MmapOptions};
use bytes::str::ByteStr;
use std::io::Read;
use std::{fs::{OpenOptions, File}, io::{Seek, SeekFrom, Write}, os::unix::prelude::AsRawFd, ptr, fs, mem};
use std::convert::TryInto;
use bytes::{MutBuf, ToBytes};
use regex::bytes::Regex;
use std::borrow::Borrow;
use std::cmp::min;
use core::cmp;

struct Node {
    min_ip: u32,
    max_ip: u32,
    left: usize,
    right: usize,
    name: [u8; 32],
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

fn node_from_bytes(slice: &[u8]) -> &Node { unsafe { bytes_to_typed(slice) } }

pub unsafe fn bytes_to_typed<T>(slice: &[u8]) -> &T {
    std::slice::from_raw_parts(slice.as_ptr() as *const T, std::mem::size_of::<T>())
        .get(0)
        .unwrap()
}

fn node_to_bytes(node: &Node) -> &[u8] { unsafe { any_as_u8_slice(node) } }

fn main() {
    run(SOURCE_PATH);
}

fn run(scr: &str) {
    fs::remove_file(MAP_PATH);

    let mut mmap = get_memmap();

    let buffer = get_buffer(scr);

    let mut shifter = 0;

    for (_, line) in buffer.lines().enumerate() {
        if line.is_err() { continue }
        let l = line.unwrap();
        if l.is_empty() { continue; }

        let node = get_node_for_line(l);
        if node.is_none() { continue }

        place_item(& mut mmap, & mut shifter, &node.unwrap());
    }
}

fn get_node_for_line(l: String) -> Option<Node> {
    let ipRegex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
    let nameRegex = Regex::new(r"\b([A-z]|[A-z]\s)+[A-z]\b").unwrap();
    let min_ip_match = ipRegex.find(l.as_bytes()).expect("didnt find min ip");
    let max_ip_match = ipRegex.find_at(l.as_bytes(),min_ip_match.end())?;
    let name_match = nameRegex.find_at(l.as_bytes(),max_ip_match.end())?;

    println!("min:{}- max:{}- name:{}", &l[min_ip_match.range()], &l[max_ip_match.range()], &l[name_match.range()]);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, name_match.as_bytes());

    let min_ip = get_u32_for_ip(&l[min_ip_match.range()])?;
    let max_ip = get_u32_for_ip(&l[max_ip_match.range()])?;

    Some(Node { min_ip, max_ip, left: 0, right: 0, name, })
}

fn get_u32_for_ip(v: &str ) -> Option<u32> {
    let v: Vec<&str> = v.split('.').collect();
    if v.len() < 4 { return None }
    let mut minArray: [u8; 4] = Default::default();
    for i in 0..v.len() {
        minArray[i] = match v[i].parse() {
            Ok(n) => n,
            Err(e) => return None
        }
    }
    //println!("IP?{}.{}.{}.{}",minArray[0],minArray[1],minArray[2],minArray[3]);
    Some(u32::from_be_bytes(minArray))
}

fn insert_array_in_array(one: & mut [u8; 32], two: &[u8])  {
    for (place, data) in one.iter_mut().zip(two.iter()) {
        *place = *data
    }
}

fn place_item(
    mmap: & mut MmapMut,
    offset: & mut usize,
    node: & Node,
) {
    let bytes = node_to_bytes(node);
    mmap[*offset..(*offset+bytes.len())].copy_from_slice(bytes);
    //println!("offset:{} {:?}", *offset, bytes);
    //println!("mmap: {:?}", &mmap[0..40]);
    *offset += bytes.len();
}

fn get_items<'a>(mmap: &'a MmapMut, offset: usize) -> &'a Node {
    let byteMap = &mmap[offset..(offset+NODE_SIZE)];
    //let byteMap = unsafe { std::slice::from_raw_parts(&mmap+offset, NODE_SIZE) };
    let node = node_from_bytes(&byteMap);
    //print_map(&mmap);
    //println!("første dims: {:?}", &byteMap);
    &node
}

fn find_node(ip: u32) -> Option<[u8; 32]> {
    let mut counter: usize = 0;
    let mut mmap = get_memmap();
    while counter < 50 {
        let node = get_items(&mmap, NODE_SIZE*counter);
        if node.min_ip < ip && ip < node.max_ip { return Some(node.name) }
        counter += 1;
    }
    None
}

fn get_buffer(file: &str) -> BufReader<std::fs::File> {
    BufReader::new(File::open(file).expect("could not find file"))
}

fn get_memmap() -> MmapMut {
    const SIZE: u64 = 128 * 128;
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
fn test_find() {
    fs::remove_file(MAP_PATH);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let min_ip = 20;
    let max_ip = 40;

    let node1 = Node { min_ip, max_ip: 0, left: 0, right: 0, name: Default::default(), };
    let node2 = Node { min_ip, max_ip: 40, left: 0, right: 0, name, };
    let node3 = Node { min_ip, max_ip: 0, left: 0, right: 0, name: Default::default(), };

    let mut firstMap = get_memmap();
    place_item(& mut firstMap, &mut 0, &node1);
    place_item(& mut firstMap, &mut NODE_SIZE, &node2);
    place_item(& mut firstMap, &mut (NODE_SIZE*2), &node3);

    let mut anotherMap = get_memmap();
    let gottenName = find_node((min_ip+max_ip)/2).unwrap();

    assert_eq!(gottenName, name);
}

#[test]
fn test_place_item_and_get() {
    fs::remove_file(MAP_PATH);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node = Node {
        min_ip: 20,
        max_ip: 20,
        left: 0,
        right: 0,
        name: name,
    };

    let mut firstMap = get_memmap();
    place_item(& mut firstMap, &mut 0, &node);

    let mut anotherMap = get_memmap();
    let getnode = get_items(&anotherMap, 0);

    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getnode.name).unwrap();

    assert_eq!(left, right);
}

#[test]
fn test_correct_layering() {
    fs::remove_file(MAP_PATH);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name, };

    let mut firstMap = get_memmap();
    place_item(& mut firstMap, &mut 0, &node);
    place_item(& mut firstMap, &mut NODE_SIZE, &node);

    let mut anotherMap = get_memmap();
    let getnode = get_items(&anotherMap, NODE_SIZE);

    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getnode.name).unwrap();
    assert_eq!(left, right);
}

#[test]
#[should_panic]
fn test_correct_layering_panic() {
    fs::remove_file(MAP_PATH);

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node = Node { min_ip: 20, max_ip: 20, left: 0, right: 0, name: name, };

    let mut firstMap = get_memmap();
    place_item(& mut firstMap, &mut 0, &node);

    let mut anotherMap = get_memmap();
    //get something that doesnt exists
    let getnode = get_items(&anotherMap, NODE_SIZE);

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