const mapPath : &str = "testdata/db.txt";
const sourcePath : &str = "testdata/set1.txt";
const nodeSize : usize = std::mem::size_of::<Node>();

use std;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::collections::HashMap;
use memmap::{MmapMut, MmapOptions};
use bytes::str::ByteStr;
use std::io::Read;
use std::{fs::{OpenOptions, File}, io::{Seek, SeekFrom, Write}, os::unix::prelude::AsRawFd, ptr, fs, mem};
use std::convert::TryInto;
use bytes::MutBuf;
use regex::bytes::Regex;
use std::borrow::Borrow;

struct Node {
    //size: u32,
    min_ip: u32,
    max_ip: u32,
    //    left: u128,
//    right: u128,
    name: u32,
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
    run();
}

fn run() {
    fs::remove_file(mapPath);

    let mut mmap = get_memmap();

    let buffer = get_buffer(sourcePath);

    let mut shifter = 0;

    for (_, line) in buffer.lines().enumerate() {
        let l = line.unwrap();
        if l.is_empty() { continue; }

        let ipRegex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
        let nameRegex = Regex::new(r"\b([A-z]|[A-z]\s)+[A-z]\b").unwrap();
        let minIp = ipRegex.find(l.as_bytes()).expect("didnt find min ip");
        let maxIp = ipRegex.find_at(l.as_bytes(),minIp.end()).expect("didnt find max ip");
        let name = nameRegex.find_at(l.as_bytes(),maxIp.end()).expect("didnt find name");


        //let s = std::str::from_utf8(&minIp.as_bytes()).unwrap();
        //print!("{}", s);



        let v: Vec<&str> = l.split(' ').collect();

        let min = v[0];
        let max = v[1];
        let com = v[2];

        let node = Node {
            min_ip: 20,
            max_ip: 20,
            name: 20,
        };

        place_item(& mut mmap, & mut shifter, &node);
    }

    print_map(&mmap);
}

fn place_item(
    mmap: & mut MmapMut,
    offset: & mut usize,
    node: & Node,
) {
    let bytes = node_to_bytes(node);
    mmap[*offset..(*offset+bytes.len())].copy_from_slice(bytes);
    println!("offset:{} {:?}", *offset, bytes);
    println!("mmap: {:?}", &mmap[0..40]);
    *offset += bytes.len();
}

fn get_items<'a>(mmap: &'a MmapMut, offset: usize) -> &'a Node {
    let byteMap = &mmap[offset..(offset+nodeSize)];
    //let byteMap = unsafe { std::slice::from_raw_parts(&mmap+offset, nodeSize) };
    let node = node_from_bytes(&byteMap);
    //print_map(&mmap);
    println!("første dims: {:?}", &byteMap);
    &node
}

fn print_map(map: &[u8]) {
    let s = std::str::from_utf8(map).unwrap();
    print!("{}", s);
}

fn get_buffer(name: &str) -> BufReader<std::fs::File> {
    BufReader::new(File::open(name).expect("could not find file"))
}

fn get_memmap() -> MmapMut {
    const SIZE: u64 = 128 * 128;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(mapPath)
        .expect("Unable to open file");
    file.seek(SeekFrom::Start(SIZE)).unwrap();
    file.write_all(&[0]).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut mmap = unsafe { MmapOptions::new().map_mut( & file).unwrap() };
    mmap
}


#[test]
fn node_to_from_bytes() {
    //let test = "a".as_bytes();

    //let lort = u32::from_be_bytes(test);

    let col = Node { min_ip: 10, max_ip: 20, name: 20 };
    let bytes = unsafe { any_as_u8_slice(&col) };
    println!("{:?}", bytes);
    println!("{}", bytes.len());
    assert_eq!(bytes, [10, 0, 0, 0, 20, 0, 0, 0, 20, 0, 0, 0]);
    let node = node_from_bytes(bytes);
    assert_eq!(20, node.name)
}


#[test]
fn get_works() {
    assert!(true);
    println!("hej");
    fs::remove_file(mapPath);
    let mut mmap = get_memmap();
    println!("hej");

    assert!(true);

    let node = Node {
        min_ip: 20,
        max_ip: 20,
        name: 20,
    };

    println!("{:?}", node_to_bytes(&node));

    place_item(& mut mmap, &mut 0, &node);


    let offset = 27;
    let getnode = get_items(&mmap, 0);
    println!("mmap: {:?}", &mmap[0..40]);
    assert_eq!(getnode.name, 20)
}

#[test]
fn testtest() {
    let string = b"195.249.336.433 195.249.336.433 Hans Hansens Hus";
    let r = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})|(\w+\s?)+").unwrap();
    let test = r.find(string).unwrap();
    let s = std::str::from_utf8( &string[test.range()]).unwrap();
    print!("{}", s);
}

//(\d{1,3}[.]){3}(\d{1,3})|(\w+\s?)+
//(\d{1,3}[.]){3}(\d{1,3})\s
