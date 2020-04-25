#![allow(box_pointers)]
#![allow(irrefutable_let_patterns)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_assignments)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const test_set_1:               &str    = "testdata/in/set1.txt";
const test_set_2:               &str    = "testdata/in/set2.txt";
const test_set_3:               &str    = "testdata/in/set3.txt";
const test_set_4:               &str    = "testdata/in/set4.txt";
const test_set_5:               &str    = "testdata/in/10_000.txt";
const test_set_6:               &str    = "testdata/in/50_000.txt";
const TREE_PRINT_PATH:          &str    = "testdata/out/tree/tree_print.txt";
const TREE_PATH:                &str    = "testdata/out/tree/map.txt";
const TREE_PAYLOAD:             &str    = "testdata/out/tree/NAME_TABLE.txt";
const REDBLACK_PRINT_PATH:      &str    = "testdata/out/redblack/tree_print.txt";
const REDBLACK_PATH:            &str    = "testdata/out/redblack/map.txt";
const REDBLACK_PAYLOAD:         &str    = "testdata/out/redblack/NAME_TABLE.txt";
const TABLE_PATH:               &str    = "testdata/out/table/IP_TABLE.txt";
const TABLE_PAYLOAD:            &str    = "testdata/out/table/NAME_TABLE.txt";

const thisFileWillBeDeleted: &str = "thisFileWillBeDeleted";

const usizeSize:        usize   = std::mem::size_of::<usize>();
const u128Size:         usize   = std::mem::size_of::<u128>();
const u64Size:          usize   = std::mem::size_of::<u64>();
const u32Size:          usize   = std::mem::size_of::<u32>();
const u16Size:          usize   = std::mem::size_of::<u16>();
const u8Size:           usize   = std::mem::size_of::<u8>();

mod FileGenerator;
mod BST;
mod RedBlack;
mod Table;
mod IntegrationTests;
mod BenchmarkTest;
mod BenchmarkTests_Separate;
mod Utils;
mod PayloadMap;

use std::io::{LineWriter, Error, Lines};
use std::ops::Add;
use memmap::{MmapMut, MmapOptions};
use std::io::Read;
use std::{fs::{OpenOptions, File}, io::{Seek, SeekFrom, Write}, os::unix::prelude::AsRawFd, ptr, fs, mem, fmt, io};
use regex::bytes::Regex;
use std::cmp::min;
use rand::{Rng, random};
use std::io::prelude::*;
use rand::distributions::Alphanumeric;
use rand::prelude::ThreadRng;
use std::iter::{Map, FilterMap, Filter, FromIterator, Enumerate};
use crate::BenchmarkTest::build_and_search_data_structures;

pub struct Entry {
    pub min_ip: u32,
    pub max_ip: u32,
    pub payload: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}, n: {}, min: {}, max: {}", &self, self.payload, self.min_ip, self.max_ip)
    }
}

fn main() {
    Utils::make_needed_folders();
    build_and_search_data_structures();
}

fn build_data_structure(input: &str, payload_path: &str, structure: MmapMut, inserter: fn(&mut MmapMut, usize, Entry, u64)) {

    fs::remove_file(payload_path);
    let mut structure = structure;
    let mut payload_map = PayloadMap::gen_payload_map_from_path(payload_path);

    let ip_regex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
    let payload_regex = Regex::new(r"\b(([A-z]|\d)+\s?)+\b").unwrap();

    let string: String = (0..98).map(|_| '-').collect();
    println!("|{}|",string);

    let mut courser: u64 = 0;

    for (i, line) in Utils::get_buffer(input).lines().enumerate() {
        if line.is_err() { continue }
        let l = line.unwrap();
        if l.is_empty() { continue; }

        if i % (BenchmarkTest::n as usize/100 + 1) == 0 { print!("-"); io::stdout().flush(); }

        let entry = Utils::get_entry_for_line(&ip_regex, &payload_regex, &l);
        if entry.is_none() { continue }
        let entry = entry.unwrap();

        courser = PayloadMap::place_payload(&mut payload_map, courser, entry.payload.as_bytes());
        let payload_index = courser - entry.payload.len() as u64 - 1;
        inserter(&mut structure, i, entry, payload_index);
    }
}