use stopwatch::Stopwatch;
use crate::{FileGenerator, Utils, Table, PayloadMap, RedBlack, REDBLACK_PATH, BST, TREE_PATH, TABLE_PATH, TABLE_PAYLOAD, TREE_PAYLOAD, REDBLACK_PAYLOAD};
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
use crate::BenchmarkTest::{search_time, range, input_data, n, input_data_shuffled, payload_lenght, gap};

const benchmark_output:        &str = "testdata/out/speed/benchmark.txt";

#[test]
#[ignore]
pub fn create_test_data() {
    Utils::make_needed_folders();
    fs::remove_file(input_data);
    fs::remove_file(input_data_shuffled);

    let file = OpenOptions::new().write(true).create(true).append(true).open(benchmark_output).unwrap();
    let mut line_writer = LineWriter::new(file);
    line_writer.write_all(format!("Benchmark input: n: {}, range: {:#?}, payload_size: {}, gap: {} \n\n", &n, &range, &payload_lenght, &gap).as_bytes());

    println!("## create_test_data");
    FileGenerator::generate_source_file_equal_spread(input_data, n, range, payload_lenght);
}

#[test]
#[ignore]
fn create_table() {
    println!("\n## load_to_table");
    let mut sw = Stopwatch::start_new();
    Table::build(input_data_shuffled);
    sw.stop();
    println!("\ntable load time: {}  micro seconds", sw.elapsed().as_micros());
}

#[test]
#[ignore]
fn create_BST() {
    println!("\n## load_to_tree");
    let mut sw = Stopwatch::start_new();
    BST::build(input_data_shuffled);
    sw.stop();
    println!("\ntree load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

#[test]
#[ignore]
fn create_redblack() {
    println!("\n## load_to_redblack");
    let mut sw = Stopwatch::start_new();
    RedBlack::build(input_data_shuffled);
    sw.stop();
    println!("\nredblack load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

#[test]
#[ignore]
fn search_time_table() {
    sleep(time::Duration::from_secs(1));
    println!("\n## search_time_table");
    println!("{}",search_time(TABLE_PAYLOAD, Table::gen_ip_table, Table::find_value_on_map));
}

#[test]
#[ignore]
fn search_time_BST() {
    println!("\n## search_time_BST");
    println!("{}",search_time(TREE_PAYLOAD, BST::gen_tree_map, BST::find_value_on_map));
}

#[test]
#[ignore]
fn search_time_redblack() {
    sleep(time::Duration::from_secs(1));
    RedBlack::load_root_node(REDBLACK_PATH);
    println!("\n## search_time_redblack");
    println!("{}",search_time(REDBLACK_PAYLOAD, RedBlack::gen_tree_map, RedBlack::find_value_on_map));
}
