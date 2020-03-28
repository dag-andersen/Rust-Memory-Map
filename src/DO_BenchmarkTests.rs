use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, TREE_PATH, load_to_tree_on_path, load_to_table, Utils, NAME_TABLE, IP_TABLE, u32Size, SP_100_000, SP_10_000, thisFileWillBeDeleted, Table, SP_500_000, SP_50_000, NameTable, load_to_tree, RedBlack, load_to_redblack};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::Tree;
use crate::Tree::TreePrinter;
use crate::Utils::get_memmap;
use crate::FileGenerator::generate_source_file_with;

const DO_Benchmark_test_pre:   &str = "DO_Benchmark_test_pre.txt";
const DO_Benchmark_test_src:   &str = "DO_Benchmark_test.txt";
const benchmark_output:        &str = "testdata/out/speed/benchmark.txt";

#[test]
#[ignore]
fn create_test_data() {

    let n = 150_000_000;
    let range = 10..18;
    let padding = 10..18;
    let name = 2;

    println!("## create_test_data");
    let src = DO_Benchmark_test_pre;

    let file = File::create(benchmark_output).unwrap();
    let mut line_writer = LineWriter::new(file);
    line_writer.write_all(format!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, namesize: {} \n\n", &n, &range, &padding, &name).as_bytes());

    fs::remove_file(src);
    FileGenerator::generate_source_file_with(src, n,range,padding, name);
}

#[test]
#[ignore]
fn build_tree() {
    println!("## build_tree");
    load_to_tree(DO_Benchmark_test_src);
}

#[test]
#[ignore]
fn build_table() {
    println!("## load_to_table");
    load_to_table(DO_Benchmark_test_src);
}

#[test]
#[ignore]
fn build_redblack() {
    println!("## load_to_table");
    load_to_redblack(DO_Benchmark_test_src);
}

#[test]
#[ignore]
fn search_time_tree() {
    println!("## search_time_tree");
    let src = DO_Benchmark_test_src;

    let requests = FileGenerator::generate_lookup_testdata(src,50);
    let length = requests.len();
    assert!(length > 0);

    let mmap = Tree::gen_tree_map();
    let name_table = NameTable::gen_name_table();
    let mut numberSkipped = 0;

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Tree::find_value_on_map(ip, &mmap, &name_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                numberSkipped += 1;
                //println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
            }
        } else { numberSkipped += 1; println!("Found none - real name: {} - ip: {}", name, ip) }
    }
    println!("--- Tree : #{} micro seconds, #{} of requests ran, #{} skipped\n", sw.elapsed().as_micros(), length, numberSkipped);
}

#[test]
#[ignore]
fn search_time_redblack() {
    println!("## search_time_redblack");
    let src = DO_Benchmark_test_src;

    let requests = FileGenerator::generate_lookup_testdata(src,50);
    let length = requests.len();
    assert!(length > 0);

    let mmap = RedBlack::gen_tree_map();
    let name_table = NameTable::gen_name_table();
    let mut numberSkipped = 0;

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = RedBlack::find_value_on_map(ip, &mmap, &name_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                numberSkipped += 1;
                //println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
            }
        } else { numberSkipped += 1; /*println!("Found none - real name: {} - ip: {}", name, ip) */}
    }
    println!("--- RedBlack : #{} micro seconds, #{} of requests ran, #{} skipped\n", sw.elapsed().as_micros(), length, numberSkipped);
}

#[test]
#[ignore]
fn search_time_table() {
    println!("## search_time_table");
    let src = DO_Benchmark_test_src;

    let requests = FileGenerator::generate_lookup_testdata(src,1000);
    let length = requests.len();
    assert!(length > 0);

    let name_table = NameTable::gen_name_table();
    let ip_table = Table::gen_ip_table();
    let mut numberSkipped = 0;

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Table::find_value_on_map(ip, &ip_table, &name_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                numberSkipped += 1;
                //println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
            }
        } else { numberSkipped += 1; println!("Found none - real name: {} - ip: {}", name, ip) }
    }
    sw.stop();
    println!("--- table : #{} micro seconds, #{} of requests ran, #{} skipped\n", sw.elapsed().as_micros(), length, numberSkipped);
}
