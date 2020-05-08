use stopwatch::Stopwatch;
use crate::{FileGenerator, Utils, Table, PayloadMap, RedBlack, BST, number_of_entries};
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
    search_time(input, gap, Table::PAYLOAD, Table::gen_ip_table, Table::find_value_on_map)
}

pub fn search_time_BST(input: &str, gap: usize) -> String {
    sleep(time::Duration::from_secs(1));
    println!("\n## search_time_BST");
    search_time(input, gap, BST::PAYLOAD, BST::gen_tree_map, BST::find_value_on_map)
}

pub fn search_time_redblack(input: &str, gap: usize) -> String {
    RedBlack::load_root_node();
    println!("\n## search_time_redblack");
    search_time(input, gap, RedBlack::PAYLOAD, RedBlack::gen_tree_map, RedBlack::find_value_on_map)
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
    let payload_map = PayloadMap::gen_payload_map_from_path(payload_path);
    let mut noneFound = 0;
    let mut wrongFound = 0;

    let string: String = (0..98).map(|_| '-').collect();
    println!("|{}|",string);

    let mut i = 0;
    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = finder(ip, &structure, &payload_map);
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