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
mod Utils;
mod PayloadMap;

use std::io::{LineWriter, Error, Lines};
use std::ops::{Add, Range};
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
use crate::BenchmarkTest::{create_table, shuffle_file, create_redblack, create_BST, search_time_BST, search_time_redblack, search_time_table};
use clap::{Arg, App, SubCommand};
use std::process::{Command, exit};
use std::thread::sleep;
use core::time;
use crate::Utils::get_u32_for_ip;

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

pub const input_data:               &str = "input_data.txt";
pub const input_data_shuffled:      &str = "input_data_shuffled.txt";

pub static mut number_of_entries:    u32 = 150_000_000;
pub const range:              Range<u32> = 10..18;
pub const shuffle_in_momory:        bool = false;

fn main() {
    Utils::make_needed_folders();

    let matches = App::new("Rust Memory Map")
        .version("1.0.0")
        .author("Dag Andersen <daga@itu.dk>")
        .about("Searching in memory mapped files")
        .arg(Arg::with_name("number_of_entries")
            .short("n")
            .long("number_of_entries")
            .takes_value(true)
            .help("The number of entries"))
        .arg(Arg::with_name("payload_size")
            .short("p")
            .long("payload_size")
            .takes_value(true)
            .help("The number of bytes for each entry"))
        .arg(Arg::with_name("gap_size")
            .short("g")
            .long("gap_size")
            .takes_value(true)
            .help("The number of entries it skips while selecting/collecting which IPs to search for"))
        .arg(Arg::with_name("input_file")
            .short("i")
            .long("input_file")
            .takes_value(true)
            .help("The file for building the data structure"))
        .arg(Arg::with_name("specific_ip")
            .short("s")
            .long("specific_ip")
            .takes_value(true)
            .help("The specific IP you want to search for"))
        .arg(Arg::with_name("generate_data")
            .short("G")
            .long("generate_data")
            .help("Generates random entries instead of getting the input from a file"))
        .arg(Arg::with_name("print_info")
            .long("print_info")
            .help("Prints the setup for this run"))
        .arg(Arg::with_name("build_table")
            .short("t")
            .long("build_table")
            .help("Builds a Table for given input"))
        .arg(Arg::with_name("build_BST")
            .short("b")
            .long("build_BST")
            .help("Builds a BST for given input"))
        .arg(Arg::with_name("build_redblack")
            .short("r")
            .long("build_redblack")
            .help("Builds a Redblack Tree for given input"))
        .arg(Arg::with_name("search_table")
            .short("T")
            .long("search_table")
            .help("Searches the Table with <number_of_entries / gap_size> number of entries"))
        .arg(Arg::with_name("search_BST")
            .short("B")
            .long("search_BST")
            .help("Searches down the BST with <number_of_entries / gap_size> number of entries"))
        .arg(Arg::with_name("search_redblack")
            .short("R")
            .long("search_redblack")
            .help("Searches down the Redblack Tree with <number_of_entries / gap_size> number of entries"))
        .get_matches();

    let shuffled_file_str = matches.value_of("input_file");
    let generate_data = matches.is_present("generate_data");
    let specific_ip = matches.value_of("specific_ip");
    let build_BST = matches.is_present("build_BST");
    let build_redblack = matches.is_present("build_redblack");
    let build_table = matches.is_present("build_table");
    let search_BST = matches.is_present("search_BST");
    let search_redblack = matches.is_present("search_redblack");
    let search_table = matches.is_present("search_table");

    if !search_BST && !search_redblack && !search_table && specific_ip.is_some() {
        println!("You have so specify which data structure you want to search in");
        exit(0)
    }

    if !search_BST && !search_redblack && !search_table && specific_ip.is_some() {
        println!("You have so specify which data structure you want to search in");
        exit(0)
    }

    let n = match matches.value_of("number_of_entries") {
        None => 150_000_000,
        Some(s) => s.parse::<u32>().unwrap_or(150_000_000)
    };
    unsafe { number_of_entries = n }

    let piece = (std::u32::MAX as f64 / n as f64) as u32;
    let padding = piece - range.end..piece - range.start;

    let payload_size = match matches.value_of("payload_size") {
        None => 50,
        Some(s) => s.parse::<usize>().unwrap_or(50)
    };

    let gap_size = match matches.value_of("gap_size") {
        None => 10,
        Some(s) => s.parse::<usize>().unwrap_or(10)
    };

    if matches.is_present("print_info") {
        println!("\nHOSTNAME: {}", String::from_utf8(Command::new("hostname").output().unwrap().stdout).unwrap());
        println!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, payload_size: {}, gap:{} \n\n", &n, &range, &padding, payload_size, &gap_size);
    }

    let input = match (shuffled_file_str, generate_data, specific_ip) {
        (None,true,_)  => {
            fs::remove_file(input_data);
            fs::remove_file(input_data_shuffled);
            if shuffle_in_momory {
                FileGenerator::generate_source_file_shuffled(input_data, n, range, padding, payload_size);
            } else {
                FileGenerator::generate_source_file(input_data, n, range, padding, payload_size);
                sleep(time::Duration::from_secs(1));
                shuffle_file(input_data, input_data_shuffled);
            }
            Some(input_data_shuffled)
        },
        (None,false,None) => {
            println!("You have to specify a input file or add the flag --generate_data");
            exit(0)
        },
        (Some(f),true,_) => {
            println!("You cant both generate data and provide an input file");
            exit(0)
        },
        (f,false,_) => f,
    };

    if (build_BST || build_redblack || build_table) && input.is_none() {
        println!("You cant build without specifying generating data or providing a file");
        exit(0)
    }

    if input.is_some() && build_BST         { create_BST(input.unwrap()); }
    if input.is_some() && build_redblack    { create_redblack(input.unwrap()); }
    if input.is_some() && build_table       { create_table(input.unwrap()); }

    match (search_BST, specific_ip, input) {
        (true,Some(s),_)    => println!("{}",BST::find_value(get_u32_for_ip(s).expect("Invalid IP")).unwrap_or("Nothing found".to_string())),
        (true,None,Some(i)) => println!("{}",search_time_BST(i,gap_size)),
        _ => {}
    }

    match (search_redblack, specific_ip, input) {
        (true,Some(s),_)    => println!("{}",RedBlack::find_value(get_u32_for_ip(s).expect("Invalid IP")).unwrap_or("Nothing found".to_string())),
        (true,None,Some(i)) => println!("{}",search_time_redblack(i,gap_size)),
        _ => {}
    }

    match (search_table, specific_ip, input) {
        (true,Some(s),_)    => println!("{}",Table::find_value(get_u32_for_ip(s).expect("Invalid IP")).unwrap_or("Nothing found".to_string())),
        (true,None,Some(i)) => println!("{}",search_time_table(i,gap_size)),
        _ => {}
    }
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

    let n = unsafe { number_of_entries };

    for (i, line) in Utils::get_buffer(input).lines().enumerate() {
        if line.is_err() { continue }
        let l = line.unwrap();
        if l.is_empty() { continue; }

        if i % (n as usize/100 + 1) == 0 { print!("-"); io::stdout().flush(); }

        let entry = Utils::get_entry_for_line(&ip_regex, &payload_regex, &l);
        if entry.is_none() { continue }
        let entry = entry.unwrap();

        courser = PayloadMap::place_payload(&mut payload_map, courser, entry.payload.as_bytes());
        let payload_index = courser - entry.payload.len() as u64 - 1;
        inserter(&mut structure, i, entry, payload_index);
    }
}