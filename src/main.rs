
use std;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use memmap::{MmapMut, MmapOptions};
use std::io::Read;


use std::{
    fs::{OpenOptions, File},
    io::{Seek, SeekFrom, Write},
    os::unix::prelude::AsRawFd,
    ptr,
};
use std::convert::TryInto;

fn main() {
    let file = load_file("testdata/set1.txt");
    let mut map : HashMap<String,String> = HashMap::new();
    interate(file, &mut map);
    println!("{}",map.get("Netcompany").unwrap().as_str());
}

fn load_file(name: &str) -> BufReader<std::fs::File> {
    println!("load_file Called");
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);
    reader
}

fn interate(reader: BufReader<std::fs::File>, map: &mut HashMap<String,String>) {
    println!("Interate Called");
    for (_, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        let v: Vec<&str> = l.split(' ').collect();
        let key = (v[0], v[1]);
        let valueCompany = v[2];
        add_to_map(key, valueCompany, map);
    }
}

struct Node {
    size: u128,
    min_ip: u128,
    max_ip: u128,
    left: u128,
    right: u128,
    value: u128,
}

fn add_to_map(key: (&str,&str), value: &str, map: &mut HashMap<String, String>) {
    println!("- item added {} {}", key.0, value);
    map.insert(key.0.to_string(), value.to_string());
}



#[test]
fn verify_simple_map() {
    let file = load_file("testdata/set1.txt");
    let mut map : HashMap<String,String> = HashMap::new();
    interate(file, &mut map);

    assert_eq!("Netcompany", map.get("1113333").unwrap());
}

#[test]
fn test2() {
    let test = std::mem::size_of::<Node>();
    println!("{}", test);

    let mut mmap_options = MmapOptions::new();
    let mut mmap: MmapMut = mmap_options.len(3).map_anon().unwrap();
    mmap.copy_from_slice(b"hej");

    let test = &mmap[0..3];
    print!("{}", std::str::from_utf8(test).unwrap());
    

    assert!(true);
}

#[test]
fn test() {
    let file = File::open("README.md").unwrap();
    let mmap = unsafe { MmapOptions::new().map( & file).unwrap() };
    //let test = str:from_utf8_unchecked((& mmap[0..8]));
    let test = &mmap[0..8];
    print!("{}", std::str::from_utf8(test).unwrap());
}

#[test]
fn test3() {

    const SIZE: u64 = 1024 * 1024;
    let src = "Hello!";

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test.mmap")
        .expect("Unable to open file");

    // Allocate space in the file first
    f.seek(SeekFrom::Start(SIZE)).unwrap();
    f.write_all(&[0]).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    let mut data = unsafe {
        memmap::MmapOptions::new()
            .map_mut(&f)
            .expect("Could not access data from memory mapped file")
    };

    data[..src.len()].copy_from_slice(src.as_bytes());

    assert!(true);
}