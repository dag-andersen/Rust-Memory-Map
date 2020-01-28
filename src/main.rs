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

        let ipRegex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
        let nameRegex = Regex::new(r"\b([A-z]|[A-z]\s)+[A-z]\b").unwrap();
        let minIp = ipRegex.find(l.as_bytes()).expect("didnt find min ip");
        let maxIp = ipRegex.find_at(l.as_bytes(),minIp.end()).expect("didnt find max ip");
        let name = nameRegex.find_at(l.as_bytes(),maxIp.end()).expect("didnt find name");

        println!("min:{}- max:{}- name:{}", &l[minIp.range()], &l[maxIp.range()], &l[name.range()]);

        let mut a: [u8; 32] = Default::default();
        insertArrayInArray(& mut a,name.as_bytes());


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


        let v: Vec<&str> = l.split(' ').collect();

        let min = v[0];
        let max = v[1];
        let com = v[2];

        let node = Node {
            min_ip: 20,
            max_ip: 20,
            name: a,
        };

        place_item(& mut mmap, & mut shifter, &node);
    }

    print_map(&mmap);
}

fn insertArrayInArray(one: & mut [u8; 32], two: &[u8])  {
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
    insertArrayInArray(& mut name,"name".as_bytes());

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
