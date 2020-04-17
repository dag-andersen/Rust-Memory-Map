use stopwatch::Stopwatch;
use crate::{FileGenerator, load_to_tree_on_path, load_to_table, Utils, Table, NameTable, load_to_tree, RedBlack, load_to_redblack, REDBLACK_PATH, Tree, TREE_PATH, TABLE_PATH, TABLE_PAYLOAD, TREE_PAYLOAD, REDBLACK_PAYLOAD};
use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{LineWriter, Write};
use crate::Tree::TreePrinter;
use memmap::MmapMut;
use std::ops::Range;
use std::thread::sleep;
use core::time;
use std::process::Command;
use std::time::SystemTime;
use crate::BenchmarkTest::search_time;

const DO_Benchmark_test_pre:   &str = "DO_Benchmark_test_pre.txt";
const DO_Benchmark_test_src:   &str = "DO_Benchmark_test.txt";
const benchmark_output:        &str = "testdata/out/speed/benchmark.txt";

pub const n:                    u32 = 100_000;
const range:             Range<u32> = 10..18;
const padding:           Range<u32> = 10..18;
const nameLength:             usize = 1;
const gap:                    usize = 10;

#[test]
#[ignore]
pub fn create_test_data() {
    Utils::make_needed_folders();
    fs::remove_file(DO_Benchmark_test_pre);
    fs::remove_file(DO_Benchmark_test_src);

    let file = OpenOptions::new().write(true).create(true).append(true).open(benchmark_output).unwrap();
    let mut line_writer = LineWriter::new(file);
    line_writer.write_all(format!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, namesize: {}, gap:{} \n\n", &n, &range, &padding, &nameLength, &gap).as_bytes());

    println!("## create_test_data");
    FileGenerator::generate_source_file(DO_Benchmark_test_pre, n, range, padding, nameLength);
}

#[test]
#[ignore]
fn create_table() {
    println!("\n## load_to_table");
    let mut sw = Stopwatch::start_new();
    load_to_table(DO_Benchmark_test_src);
    sw.stop();
    println!("\ntable load time: {}  micro seconds", sw.elapsed().as_micros());
}

#[test]
#[ignore]
fn create_tree() {
    println!("\n## load_to_tree");
    let mut sw = Stopwatch::start_new();
    load_to_tree(DO_Benchmark_test_src);
    sw.stop();
    println!("\ntree load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

#[test]
#[ignore]
fn create_redblack() {
    println!("\n## load_to_redblack");
    let mut sw = Stopwatch::start_new();
    load_to_redblack(DO_Benchmark_test_src);
    sw.stop();
    println!("\nredblack load time: {}  micro seconds", sw.elapsed().as_micros());
    sleep(time::Duration::from_secs(1));
}

#[test]
#[ignore]
fn search_time_table(){
    sleep(time::Duration::from_secs(1));
    println!("\n## search_time_table");
    println!("{}",search_time(TABLE_PAYLOAD, Table::gen_ip_table, Table::find_value_on_map));
}

#[test]
#[ignore]
fn search_time_tree(){
    println!("\n## search_time_tree");
    println!("{}",search_time(TREE_PAYLOAD, Tree::gen_tree_map, Tree::find_value_on_map));
}

#[test]
#[ignore]
fn search_time_redblack() {
    sleep(time::Duration::from_secs(1));
    RedBlack::load_root_node(REDBLACK_PATH);
    println!("\n## search_time_redblack");
    println!("{}",search_time(REDBLACK_PAYLOAD, RedBlack::gen_tree_map, RedBlack::find_value_on_map));
}