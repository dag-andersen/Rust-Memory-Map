
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

struct Node {
    size: u128,
    min_ip: u128,
    max_ip: u128,
    left: u128,
    right: u128,
    value: u128,
}

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
    let src = "hej";

    let mut mmap_options = MmapOptions::new();
    let mut mmap: MmapMut = mmap_options.len(10).map_anon().unwrap();
    mmap[..src.len()].copy_from_slice(src.as_bytes());

    let test = &mmap[0..4];
    print!("{}", std::str::from_utf8(test).unwrap());

    assert!(true);
}

fn get_memmap() -> MmapMut {
    const SIZE: u64 = 128 * 128;
    let src = "Hello!";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("testdata/db.txt")
        .expect("Unable to open file");

    file.seek(SeekFrom::Start(SIZE)).unwrap();
    file.write_all(&[0]).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut mmap = unsafe { MmapOptions::new().map_mut( & file).unwrap() };
    //let test = str:from_utf8_unchecked((& mmap[0..8]));
    mmap[..src.len()].copy_from_slice(src.as_bytes());
    let test = &mmap[0..3];
    print!("{}", std::str::from_utf8(test).unwrap());
    mmap
}

#[test]
fn hejhej() {
    get_memmap();
}

#[test]
fn test3() {

    const SIZE: u64 = 1024 * 1024;
    let src = "Hello!";

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("testdata/map.mmap")
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

    let test = &data[0..8];
    print!("{}", std::str::from_utf8(test).unwrap());
    assert!(true);
}