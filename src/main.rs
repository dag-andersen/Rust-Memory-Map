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

const SOURCE_PATH_1:    &str    = "testdata/in/set1.txt";
const SOURCE_PATH_2:    &str    = "testdata/in/set2.txt";
const SOURCE_PATH_3:    &str    = "testdata/in/set3.txt";
const SP_10_000:        &str    = "testdata/in/10_000.txt";
const SP_100_000:       &str    = "testdata/in/100_000.txt";
const SP_1_000_1000:    &str    = "testdata/in/1_000_000.txt";
const MAP_PATH:         &str    = "testdata/out/tree/map.txt";
const TREE_PRINT_PATH:  &str    = "testdata/out/tree/tree_print.txt";
const TABLE1:           &str    = "testdata/out/table/table1.txt";
const TABLE2:           &str    = "testdata/out/table/table2.txt";

const usizeSize:        usize   = std::mem::size_of::<usize>();
const u128Size:         usize   = std::mem::size_of::<u128>();
const u64Size:          usize   = std::mem::size_of::<u64>();
const u32Size:          usize   = std::mem::size_of::<u32>();
const u16Size:          usize   = std::mem::size_of::<u16>();
const u8Size:           usize   = std::mem::size_of::<u8>();

mod FileGenerator;
mod Tree;
mod Table;
mod BenchmarkTests;
mod Utils;

use std::io::{BufRead, BufReader, LineWriter, Error, Lines};
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
use crate::Utils::get_memmap;
use crate::Tree::NodeToMem;
use std::iter::{Map, FilterMap, Filter, FromIterator, Enumerate};

pub struct Entry {
    pub min_ip: u32,
    pub max_ip: u32,
    pub name: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}, n: {}, min: {}, max: {}", &self, self.name, self.min_ip, self.max_ip)
    }
}

fn main() {
    load_to_map(SOURCE_PATH_1, MAP_PATH,Tree::insert_entry);
    load_to_table(SOURCE_PATH_1);
}

fn load_to_map(input: &str, map_path: &str, map_fn: fn(&mut MmapMut, usize, Entry)) {
    fs::remove_file(map_path);

    let mut mmap = get_memmap(map_path, 3_000_000_000);

    let ip_regex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
    let name_regex = Regex::new(r"\b(([A-z]|\d)+\s?)+\b").unwrap();

    for (i, line) in get_buffer(input).lines().enumerate() {
        if line.is_err() { continue }
        let l = line.unwrap();
        if l.is_empty() { continue; }

        let entry = Utils::get_entry_for_line(&ip_regex, &name_regex, &l);
        if entry.is_none() { continue }
        let entry = entry.unwrap();

        map_fn(& mut mmap, i, entry);
    }
}

fn load_to_table(input: &str) {

    let bufReader_to_strings = (|b:BufReader<File>| {
        b.lines().map(|y| {
            if y.is_err() { return None }
            let y = y.unwrap();
            if y.is_empty() { return None }
            return Some(y);
        }).filter(|x| x.is_some()).map(|x| x.unwrap())
    });

    let mut strings_to_entries = (|x:BufReader<File>| {
        let ip_regex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
        let name_regex = Regex::new(r"\b(([A-z]|\d)+\s?)+\b").unwrap();
        bufReader_to_strings(x)
            .map(move |x| Utils::get_entry_for_line(&ip_regex, &name_regex, &x))
            .filter(|x| x.is_some())
            .map(|x|x.unwrap())
    });

    fs::remove_file(TABLE1);
    fs::remove_file(TABLE2);

    let mut lookup_table = get_memmap(TABLE1, 4_000_000_000);
    let mut ip_table = get_memmap(TABLE2, 16_000_000_000);

    let mut courser= 0;
    for entry in strings_to_entries(get_buffer(input)) {
        Table::TableLookup::place_entry(&mut lookup_table, &entry,courser as u32);
        courser = Table::TableLookup::place_name(&mut ip_table, courser, entry.name.as_bytes());
    }
}

fn get_buffer(file: &str) -> BufReader<std::fs::File> {
    BufReader::new(File::open(file).expect("could not find file"))
}

#[test]
fn test_print_tree_to_file() {
    let src = "thisFileWillBeDeleted";
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    load_to_map(src, MAP_PATH, Tree::insert_entry);
    Tree::TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}

#[test]
fn test_find_node_in_tree() {

    let insert: fn(&mut MmapMut, usize, Entry) = Tree::insert_entry;
    let get: fn(ip: u32) -> Option<[u8; 32]> = Tree::find_value;

    load_to_map(SOURCE_PATH_1,MAP_PATH, insert);

    let name = get(Utils::get_u32_for_ip("000.000.000.015").unwrap());
    assert!(name.is_some());
    let name = name.unwrap();
    let strName = std::str::from_utf8(&name).unwrap().trim_matches(char::from(0));
    assert_eq!(strName,"Siteimprove");

    let name = get(Utils::get_u32_for_ip("000.000.002.015").unwrap());
    assert!(name.is_some());
    let name = name.unwrap();
    let strName = std::str::from_utf8(&name).unwrap().trim_matches(char::from(0));
    assert_eq!(strName,"Olesen");

    let name = get(Utils::get_u32_for_ip("000.000.000.001").unwrap());
    assert!(name.is_none());

    let name = get(Utils::get_u32_for_ip("001.000.000.000").unwrap());
    assert!(name.is_none());
}

#[test]
fn test_find_inserted_node_in_tree() {

    let insert: fn(&mut MmapMut, usize, Entry) = Tree::insert_entry;
    let get: fn(ip: u32) -> Option<[u8; 32]> = Tree::find_value;

    let src = "test_find_random";
    let numberOfLines = 100;
    FileGenerator::generate_source_file(numberOfLines, src);
    load_to_map(src, MAP_PATH, insert);

    let mut name: [u8; 32] = Default::default();
    Utils::insert_array_in_array(& mut name, "testname".as_bytes());

    let ip = 34568;

    let entry = Entry { min_ip: ip-1, max_ip: ip+1, name: String::from("testname") };

    let mut mmap = get_memmap(MAP_PATH, 300000000);
    insert(&mut mmap, numberOfLines, entry);

    let getNode = get(ip);
    assert!(getNode.is_some());
    let getNode = getNode.unwrap();
    let left = std::str::from_utf8(&name).unwrap();
    let right = std::str::from_utf8(&getNode).unwrap();
    assert_eq!(left, right);

    fs::remove_file(src);
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