use stopwatch::Stopwatch;
use crate::{FileGenerator, load_to_tree_on_path, load_to_table, Utils, Table, NameTable, load_to_tree, RedBlack, load_to_redblack, REDBLACK_PATH, Tree};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{LineWriter, Write};
use crate::Tree::TreePrinter;
use memmap::MmapMut;
use std::ops::Range;

const DO_Benchmark_test_pre:   &str = "DO_Benchmark_test_pre.txt";
const DO_Benchmark_test_src:   &str = "DO_Benchmark_test.txt";
const benchmark_output:        &str = "testdata/out/speed/benchmark.txt";

const n:                        u32 = 1_000_000;
const range:             Range<u32> = 1..1;
const padding:           Range<u32> = 40..40;
const nameLength:             usize = 2;
const gap:                    usize = 100;

#[test]
#[ignore]
fn create_test_data() {

    println!("## create_test_data");
    let src = DO_Benchmark_test_pre;

    let file = OpenOptions::new().write(true).append(true).open(benchmark_output).unwrap();
    let mut line_writer = LineWriter::new(file);
    line_writer.write_all(format!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, namesize: {} \n\n", &n, &range, &padding, &nameLength).as_bytes());

    fs::remove_file(src);
    FileGenerator::generate_source_file_with(src, n, range, padding, nameLength);
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
    search_time(Tree::gen_tree_map, Tree::find_value_on_map);
}

#[test]
#[ignore]
fn search_time_redblack() {
    println!("## search_time_redblack");
    RedBlack::load_root_node(REDBLACK_PATH);
    search_time(RedBlack::gen_tree_map, RedBlack::find_value_on_map);
}

#[test]
#[ignore]
fn search_time_table() {
    println!("## search_time_table");
    search_time(Table::gen_ip_table, Table::find_value_on_map);
}

fn search_time(structure: fn() -> MmapMut, finder: fn(u32, &MmapMut, &MmapMut) -> Option<String>) {
    let src = DO_Benchmark_test_src;

    let requests = FileGenerator::generate_lookup_testdata(src,gap);
    let length = requests.len();
    assert!(length > 0);

    let structure = structure();
    let name_table = NameTable::gen_name_table();
    let mut numberSkipped = 0;

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = finder(ip, &structure, &name_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                numberSkipped += 1;
                //println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
            }
        } else { numberSkipped += 1; println!("Found none - real name: {} - ip: {}", name, ip) }
    }
    sw.stop();
    println!("Search time --- #{} micro seconds, #{} of requests ran, #{} skipped\n", sw.elapsed().as_micros(), length, numberSkipped);
}