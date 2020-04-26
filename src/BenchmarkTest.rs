use stopwatch::Stopwatch;
use crate::{FileGenerator, Utils, Table, PayloadMap, RedBlack, REDBLACK_PATH, BST, TREE_PATH, TABLE_PATH, TABLE_PAYLOAD, TREE_PAYLOAD, REDBLACK_PAYLOAD, number_of_entries};
use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{LineWriter, Write};
use crate::BST::TreePrinter;
use memmap::MmapMut;
use std::ops::Range;
use std::thread::sleep;
use core::time;
use std::process::{Command, exit};
use std::time::SystemTime;
use clap::{App, Arg};

pub const input_data:               &str = "input_data.txt";
pub const input_data_shuffled:      &str = "input_data_shuffled.txt";

pub const range:              Range<u32> = 10..18;
pub const shuffle_in_momory:        bool = false;

pub fn build_and_search_data_structures() {
    let matches = App::new("Rust Memory Map")
        .version("0.1.0")
        .author("Dag Andersen <dagbjerreandersen@gmail.com>")
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
            .help("The amount of bytes for each entry"))
        .arg(Arg::with_name("gap_size")
            .short("g")
            .long("gap_size")
            .takes_value(true)
            .help("The number of entries it skips while selecting/collecting entries to search for"))
        .arg(Arg::with_name("input_file")
            .short("f")
            .long("input_file")
            .takes_value(true)
            .help("The file for building the data-structure"))
        .arg(Arg::with_name("generate_data")
            .long("generate_data")
            .help("Generates random entries instead of getting the input from a file"))
        .arg(Arg::with_name("print_info")
            .long("print_info")
             .help("Prints the setup for this run"))
        .arg(Arg::with_name("build_table")
            .long("build_table")
            .help("Builds a Table for given input"))
        .arg(Arg::with_name("build_BST")
            .long("build_BST")
            .help("Builds a BST for given input"))
        .arg(Arg::with_name("build_redblack")
            .long("build_redblack")
            .help("Builds a Redblack Tree for given input"))
        .arg(Arg::with_name("search_table")
            .long("search_table")
            .help("Searches the Table with <number_of_entries / gap_size> number of entries"))
        .arg(Arg::with_name("search_BST")
            .long("search_BST")
            .help("Searches down the BST with <number_of_entries / gap_size> number of entries"))
        .arg(Arg::with_name("search_redblack")
            .long("search_redblack")
            .help("Searches down the Redblack Tree with <number_of_entries / gap_size> number of entries"))
        .get_matches();

    let shuffled_file_str = matches.value_of("input_file");

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

    let input = match shuffled_file_str {
        Some(f) => f,
        None if matches.is_present("generate_data") => {
            fs::remove_file(input_data);
            fs::remove_file(input_data_shuffled);
            if shuffle_in_momory {
                FileGenerator::generate_source_file_shuffled(input_data, n, range, padding, payload_size);
            } else {
                FileGenerator::generate_source_file(input_data, n, range, padding, payload_size);
                sleep(time::Duration::from_secs(1));
                shuffle_file(input_data, input_data_shuffled);
            }
            input_data_shuffled
        },
        _ => {
            println!("You have to specify a input file or add the flag --generate_data");
            exit(0)
        }
    };

    if matches.is_present("build_BST") { create_BST(input); }
    if matches.is_present("build_redblack") { create_redblack(input); }
    if matches.is_present("build_table") { create_table(input); }

    if matches.is_present("search_BST") { println!("{}",search_time_BST(input,gap_size)); }
    if matches.is_present("search_redblack") { println!("{}",search_time_redblack(input,gap_size)); }
    if matches.is_present("search_table") { println!("{}",search_time_table(input,gap_size)); }
}

pub fn shuffle_file(input: &str, output: &str) {
    if cfg!(target_os = "windows") {
        println!("This program does not work on windows!");
        exit(1)
    } else if cfg!(target_os = "macos") {
        Command::new("gshuf")
            .arg(input)
            .arg("-o")
            .arg(output)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("shuf")
            .arg(input)
            .arg("-o")
            .arg(output)
            .output()
            .expect("failed to execute process")
    };
}

pub fn create_table(input: &str) {
    println!("\n## load_to_table");
    let mut sw = Stopwatch::start_new();
    Table::build(input);
    sw.stop();
    println!("\ntable load time: {}  micro seconds", sw.elapsed().as_micros());
}

pub fn create_BST(input: &str) {
    println!("\n## load_to_tree");
    let mut sw = Stopwatch::start_new();
    BST::build(input);
    sw.stop();
    println!("\ntree load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

pub fn create_redblack(input: &str) {
    println!("\n## load_to_redblack");
    let mut sw = Stopwatch::start_new();
    RedBlack::build(input);
    sw.stop();
    println!("\nredblack load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

pub fn search_time_table(input: &str, gap: usize) -> String {
    sleep(time::Duration::from_secs(1));
    println!("\n## search_time_table");
    search_time(input, gap, TABLE_PAYLOAD, Table::gen_ip_table, Table::find_value_on_map)
}

pub fn search_time_BST(input: &str, gap: usize) -> String {
    println!("\n## search_time_BST");
    search_time(input, gap, TREE_PAYLOAD, BST::gen_tree_map, BST::find_value_on_map)
}

pub fn search_time_redblack(input: &str, gap: usize) -> String {
    sleep(time::Duration::from_secs(1));
    println!("\n## search_time_redblack");
    search_time(input, gap, REDBLACK_PAYLOAD, RedBlack::gen_tree_map, RedBlack::find_value_on_map)
}

pub fn search_time(
    input: &str,
    gap: usize,
    payload_path: &str,
    structure: fn() -> MmapMut,
    finder: fn(u32, &MmapMut, &MmapMut) -> Option<String>
) -> String {
    let requests = FileGenerator::generate_lookup_testdata(input, gap);
    let length = requests.len();
    assert!(length > 0);

    let structure = structure();
    let name_table = PayloadMap::gen_payload_map_from_path(payload_path);
    let mut noneFound = 0;
    let mut wrongFound = 0;

    let string: String = (0..98).map(|_| '-').collect();
    println!("|{}|",string);

    let mut i = 0;
    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = finder(ip, &structure, &name_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                wrongFound += 1;
            }
        } else { noneFound += 1 }
        if i % (length/100 + 1) == 0 { print!("-"); io::stdout().flush(); }
        i += 1;
    }
    print!("\n");
    sw.stop();
    format!("Search time --- #{} micro seconds, #{} of requests ran, #{} none, #{} wrong", sw.elapsed().as_micros(), length, noneFound, wrongFound)
}