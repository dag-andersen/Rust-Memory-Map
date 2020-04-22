use stopwatch::Stopwatch;
use crate::{FileGenerator, load_to_tree_on_path, load_to_table, Utils, Table, PayloadMap, load_to_tree, RedBlack, load_to_redblack, REDBLACK_PATH, BST, TREE_PATH, TABLE_PATH, TABLE_PAYLOAD, TREE_PAYLOAD, REDBLACK_PAYLOAD};
use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{LineWriter, Write};
use crate::BST::TreePrinter;
use memmap::MmapMut;
use std::ops::Range;
use std::thread::sleep;
use core::time;
use std::process::Command;
use std::time::SystemTime;

pub const input_data:               &str = "input_data.txt";
pub const input_data_shuffled:      &str = "input_data_shuffled.txt";

pub const n:                         u32 = 150_000_000;
pub const range:              Range<u32> = 10..18;
pub const padding:            Range<u32> = 10..18;
pub const nameLength:              usize = 1;
pub const gap:                     usize = 10;
pub const shuffle_in_momory:        bool = false;

pub fn create_test_data() {
    fs::remove_file(input_data);
    fs::remove_file(input_data_shuffled);

    println!("\nHOSTNAME: {}",String::from_utf8(Command::new("hostname").output().unwrap().stdout).unwrap());
    println!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, payload_size: {}, gap:{} \n\n", &n, &range, &padding, &nameLength, &gap);

    println!("## create_test_data");

    if shuffle_in_momory {
        FileGenerator::generate_source_file_shuffled(input_data, n, range, padding, nameLength);
    } else {
        FileGenerator::generate_source_file(input_data, n, range, padding, nameLength);
        sleep(time::Duration::from_secs(1));
        shuffle_file(input_data, input_data_shuffled);
    }

    create_redblack();
    create_tree();
    create_table();

    let redblack_time = search_time_redblack();
    let table_time = search_time_table();

    println!();
    println!("{}",tree_time);
    println!("{}",redblack_time);
    println!("{}",table_time);
}

fn shuffle_file(input: &str, output: &str) {
    if cfg!(target_os = "windows") {
        panic!("This is a windows machine - Shame on you!")
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

fn create_table() {
    println!("\n## load_to_table");
    let mut sw = Stopwatch::start_new();
    load_to_table(input_data_shuffled);
    sw.stop();
    println!("\ntable load time: {}  micro seconds", sw.elapsed().as_micros());
}

fn create_tree() {
    println!("\n## load_to_tree");
    let mut sw = Stopwatch::start_new();
    load_to_tree(input_data_shuffled);
    sw.stop();
    println!("\ntree load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

fn create_redblack() {
    println!("\n## load_to_redblack");
    let mut sw = Stopwatch::start_new();
    load_to_redblack(input_data_shuffled);
    sw.stop();
    println!("\nredblack load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

fn search_time_table() -> String {
    sleep(time::Duration::from_secs(1));
    println!("\n## search_time_table");
    search_time(TABLE_PAYLOAD, Table::gen_ip_table, Table::find_value_on_map)
}

fn search_time_BST() -> String {
    println!("\n## search_time_BST");
    search_time(TREE_PAYLOAD, BST::gen_tree_map, BST::find_value_on_map)
}

fn search_time_redblack() -> String {
    sleep(time::Duration::from_secs(1));
    println!("\n## search_time_redblack");
    search_time(REDBLACK_PAYLOAD, RedBlack::gen_tree_map, RedBlack::find_value_on_map)
}

pub fn search_time(payload_path: &str, structure: fn() -> MmapMut, finder: fn(u32, &MmapMut, &MmapMut) -> Option<String>) -> String {
    let src = input_data_shuffled;

    let requests = FileGenerator::generate_lookup_testdata(src, gap);
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
                //println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
            }
        } else { noneFound += 1 }
        if i % (length/100 + 1) == 0 { print!("-"); io::stdout().flush(); }
        i += 1;
    }
    print!("\n");
    sw.stop();
    format!("Search time --- #{} micro seconds, #{} of requests ran, #{} none, #{} wrong", sw.elapsed().as_micros(), length, noneFound, wrongFound)
}