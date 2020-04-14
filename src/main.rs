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

const SOURCE_PATH_1:            &str    = "testdata/in/set1.txt";
const SOURCE_PATH_2:            &str    = "testdata/in/set2.txt";
const SOURCE_PATH_3:            &str    = "testdata/in/set3.txt";
const SOURCE_PATH_4:            &str    = "testdata/in/set4.txt";
const SP_10_000:                &str    = "testdata/in/10_000.txt";
const SP_50_000:                &str    = "testdata/in/50_000.txt";
const SP_100_000:               &str    = "testdata/in/100_000.txt";
const SP_500_000:               &str    = "testdata/in/500_000.txt";
const SP_1_000_000:             &str    = "testdata/in/1_000_000.txt";
const SP_5_000_000:             &str    = "testdata/in/5_000_000.txt";
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
mod Tree;
mod RedBlack;
mod Table;
mod IntegrationTests;
mod BenchmarkTest;
mod Utils;
mod NameTable;

use std::io::{BufRead, BufReader, LineWriter, Error, Lines};
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
use crate::BenchmarkTest::create_test_data;

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
    create_test_data();
}

fn load_to_tree(input: &str) { load_to_tree_on_path(input, TREE_PATH) }

fn load_to_tree_on_path(input: &str, map_path: &str) {
    fs::remove_file(map_path);
    load_to_data_structure(input, TREE_PAYLOAD, Tree::gen_tree_map_on_path(map_path), Tree::insert_entry)
}

fn load_to_redblack(input: &str) { load_to_redblacktree_on_path(input, REDBLACK_PATH) }

fn load_to_redblacktree_on_path(input: &str, map_path: &str) {
    RedBlack::reset_root_index();
    fs::remove_file(map_path);
    load_to_data_structure(input, REDBLACK_PAYLOAD, RedBlack::gen_tree_map_on_path(map_path), RedBlack::insert_entry);
    RedBlack::save_root_node(map_path);
}

fn load_to_table(input: &str) { load_to_table_on_path(input, TABLE_PATH) }

fn load_to_table_on_path(input: &str, ip_table: &str) {
    fs::remove_file(ip_table);
    load_to_data_structure(input, TABLE_PAYLOAD, Table::gen_ip_table_from_path(ip_table), Table::insert_entry)
}

fn load_to_data_structure(input: &str, payload_path: &str, structure: MmapMut, inserter: fn(&mut MmapMut, usize, Entry, usize)) {

    fs::remove_file(payload_path);
    let mut structure = structure;
    let mut name_table = NameTable::gen_name_table_from_path(payload_path);

    let ip_regex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
    let name_regex = Regex::new(r"\b(([A-z]|\d)+\s?)+\b").unwrap();

    let mut courser= 0;

    let string: String = (0..98).map(|_| '-').collect();
    println!("|{}|",string);

    for (i, line) in get_buffer(input).lines().enumerate() {
        if line.is_err() { continue }
        let l = line.unwrap();
        if l.is_empty() { continue; }

        //if i % 50_000 == 0 { print!("", i)}
        if i % (BenchmarkTest::n as usize/100 + 1) == 0 { print!("-"); io::stdout().flush(); }

        let entry = Utils::get_entry_for_line(&ip_regex, &name_regex, &l);
        if entry.is_none() { continue }
        let entry = entry.unwrap();

        courser = NameTable::place_name(&mut name_table, courser, entry.name.as_bytes());
        let something = courser - entry.name.len() - 1;
        inserter(&mut structure, i, entry, something);
    }
}

fn get_buffer(file: &str) -> BufReader<std::fs::File> {
    BufReader::new(File::open(file).expect("could not find file"))
}

#[test]
fn find_hardcoded_node_in_tree() {
    find_hardcoded_node(load_to_tree,Tree::find_value)
}

#[test]
fn find_hardcoded_node_in_redblack() {
    find_hardcoded_node(load_to_redblack,RedBlack::find_value)
}

#[test]
fn find_hardcoded_node_in_table() {
    find_hardcoded_node(load_to_table,Table::find_value)
}

fn find_hardcoded_node(loader: fn(&str), finder: fn(u32) -> Option<String>) {
    loader(SOURCE_PATH_1);

    let name = finder(Utils::get_u32_for_ip("000.000.000.015").unwrap());
    assert!(name.is_some());
    assert_eq!(name.unwrap(),"Siteimprove");

    let name = finder(Utils::get_u32_for_ip("000.000.002.015").unwrap());
    assert!(name.is_some());
    assert_eq!(name.unwrap(),"Olesen");

    assert!(finder(Utils::get_u32_for_ip("000.000.000.001").unwrap()).is_none());
    assert!(finder(Utils::get_u32_for_ip("001.000.000.000").unwrap()).is_none());
}

#[test]
fn find_random_gen_requests_in_tree() {
    find_random_gen_request(load_to_tree,Tree::find_value);
}

#[test]
fn find_random_gen_requests_in_redblack() {
    find_random_gen_request(load_to_redblack,RedBlack::find_value);
}

#[test]
fn find_random_gen_requests_in_table() {
    find_random_gen_request(load_to_table,Table::find_value);
}

fn find_random_gen_request(loader: fn(&str), finder: fn(u32) -> Option<String>) {
    let scr = SP_10_000;
    loader(scr);
    let requests = FileGenerator::generate_lookup_testdata(scr,50);

    for (ip, name) in requests {
        let value = finder(ip);
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(name, value)
    }
}