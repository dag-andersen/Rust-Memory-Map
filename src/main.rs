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
use bytes::{MutBuf, ToBytes};
use regex::bytes::Regex;
use std::borrow::Borrow;
use std::cmp::min;
use core::cmp;

struct Node {
    min_ip: u32,
    max_ip: u32,
//    left: u128,
//    right: u128,
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
    run(sourcePath);
}

fn run(scr: &str) {
    fs::remove_file(mapPath);

    let mut mmap = get_memmap();

    let buffer = get_buffer(scr);

    let mut shifter = 0;

    for (_, line) in buffer.lines().enumerate() {
        let l = line.unwrap();
        if l.is_empty() { continue; }

        let ipRegex = get_ip_regex();
        let nameRegex = get_name_regex();
        let min_ip_match = ipRegex.find(l.as_bytes()).expect("didnt find min ip");
        let max_ip_match = ipRegex.find_at(l.as_bytes(),min_ip_match.end()).expect("didnt find max ip");
        let name_match = nameRegex.find_at(l.as_bytes(),max_ip_match.end()).expect("didnt find name");

        println!("min:{}- max:{}- name:{}", &l[min_ip_match.range()], &l[max_ip_match.range()], &l[name_match.range()]);

        let mut a: [u8; 32] = Default::default();
        insert_array_in_array(& mut a, name_match.as_bytes());

        let min_ip = get_u32_for_ip(&l[min_ip_match.range()]);
        let max_ip = get_u32_for_ip(&l[max_ip_match.range()]);

        let node = Node {
            min_ip,
            max_ip,
            name: a,
        };

        place_item(& mut mmap, & mut shifter, &node);

    }

    print_map(&mmap);
}

fn get_u32_for_ip(v: &str ) -> u32 {
    let v: Vec<&str> = v.split('.').collect();
    let mut minArray: [u8; 4] = Default::default();
    for i in 0..v.len() { minArray[i] = v[i].parse().unwrap(); }
    //println!("IP?{}.{}.{}.{}",minArray[0],minArray[1],minArray[2],minArray[3]);
    u32::from_be_bytes(minArray)
}
fn get_ip_regex() -> Regex { Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap() }
fn get_name_regex() -> Regex { Regex::new(r"\b([A-z]|[A-z]\s)+[A-z]\b").unwrap() }

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
    let byteMap = &mmap[offset..(offset+nodeSize)];
    //let byteMap = unsafe { std::slice::from_raw_parts(&mmap+offset, nodeSize) };
    let node = node_from_bytes(&byteMap);
    //print_map(&mmap);
    //println!("første dims: {:?}", &byteMap);
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
fn place_item_and_get() {
    fs::remove_file(mapPath);
    let mut mmap = get_memmap();

    let mut name: [u8; 32] = Default::default();
    insert_array_in_array(& mut name, "name".as_bytes());

    let node = Node {
        min_ip: 20,
        max_ip: 20,
        name: name,
    };

    place_item(& mut mmap, &mut 0, &node);
    let getnode = get_items(&mmap, 0);
    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getnode.name).unwrap();

    println!("left: {} -- right: {}",left, right );
    assert_eq!(left,right)
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