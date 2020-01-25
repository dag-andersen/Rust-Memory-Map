
use std;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use memmap::{MmapMut, MmapOptions};
use bytes::str::ByteStr;
use std::io::Read;
use std::{fs::{OpenOptions, File}, io::{Seek, SeekFrom, Write}, os::unix::prelude::AsRawFd, ptr, fs};
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
    let mut mmap = get_memmap();

    let buffer = get_buffer("testdata/set1.txt");

    let mut shifter = 0;

    for (_, line) in buffer.lines().enumerate() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        let v: Vec<&str> = l.split(' ').collect();

        let min = v[0];
        let max = v[1];
        let com = v[2];

        placeItem(& mut mmap,& mut shifter,&com,&min,&max);
    }

    let test = &mmap[0..100];
    print!("{}", std::str::from_utf8(test).unwrap());
}

fn placeItem(
    mmap: & mut MmapMut,
    offset: & mut usize,
    com: &str,
    min: &str,
    max: &str,
) -> usize {
    let mut s = com.to_owned();
    s.push(' ');
    s.push_str(&(min.to_owned()));
    s.push(' ');
    s.push_str(&(max.to_owned()));
    s.push(' ');

    *offset += s.len();

    let test = *offset;

    mmap[test..(test+s.len())].copy_from_slice(s.as_bytes());
    s.len()
}

fn get_buffer(name: &str) -> BufReader<std::fs::File> {
    let file = File::open(name).expect("could not find file");
    let reader = BufReader::new(file);
    reader
}

fn get_memmap() -> MmapMut {
    const SIZE: u64 = 128 * 128;

    let path = "testdata/db.txt";

    fs::remove_file(path);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Unable to open file");

    file.seek(SeekFrom::Start(SIZE)).unwrap();
    file.write_all(&[0]).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut mmap = unsafe { MmapOptions::new().map_mut( & file).unwrap() };

    mmap
}
