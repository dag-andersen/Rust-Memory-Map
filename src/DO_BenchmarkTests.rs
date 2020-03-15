use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, MAP_PATH, load_to_tree, load_to_table, Utils, NAME_TABLE, IP_TABLE, u32Size, SP_100_000, SP_10_000, thisFileWillBeDeleted, Table, SP_500_000, SP_50_000, IP_TABLE_1_000_000, NAME_TABLE_1_000_000, TREE_MAP_1_000_000};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::Tree;
use crate::Tree::TreePrinter;
use crate::Utils::get_memmap;
use crate::FileGenerator::generate_source_file_with;

const DO_Benchmark_test:   &str = "DO_Benchmark_test.txt";

#[test]
fn create_test_data() {
    println!("## create_test_data");
    let src = DO_Benchmark_test;
    fs::remove_file(src);
    FileGenerator::generate_source_file_with(src, 100_000,1..1,28..28, 1);
}

#[test]
fn build_tree() {
    println!("## build_tree");
    let src = DO_Benchmark_test;
    load_to_tree(src, MAP_PATH, Tree::insert_entry);
}

#[test]
fn build_table() {
    let src = DO_Benchmark_test;
    load_to_table(src);
}

#[test]
fn search_time_tree() {
    println!("## search_time_tree_vs_table_no_file_gen");
    let src = DO_Benchmark_test;

    let requests = FileGenerator::generate_lookup_testdata(src,100);
    let length = requests.len();
    assert!(length > 0);

    let mmap = Tree::gen_tree_map();

    let mut numberSkipped = 0;

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Tree::find_value_on_map(ip, &mmap);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                numberSkipped += 1;
                //println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
            }
        } else { println!("Found none - real name: {} - ip: {}", name, ip) }
    }
    println!("--- Tree score : {}, #{} of requests ran, #{} skipped", sw.elapsed().as_micros(), length, numberSkipped);
}

#[test]
fn search_time_table() {
    println!("## search_time_tree_vs_table_no_file_gen");
    let src = DO_Benchmark_test;

    let requests = FileGenerator::generate_lookup_testdata(src,100);
    let length = requests.len();
    assert!(length > 0);

    let lookup_table = Table::gen_lookup_table();
    let ip_table = Table::gen_ip_table();
    let mut numberSkipped = 0;

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Table::find_value_on_map(ip, &lookup_table, &ip_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                numberSkipped += 1;
                //println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
            }
        } else { println!("Found none - real name: {} - ip: {}", name, ip) }
    }
    sw.stop();
    println!("--- table score : {}, #{} of requests ran, #{} skipped", sw.elapsed().as_micros(), length, numberSkipped);
}
