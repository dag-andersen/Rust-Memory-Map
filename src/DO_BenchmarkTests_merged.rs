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

const DO_Benchmark_test_pre:   &str = "DO_Benchmark_test_pre.txt";
const DO_Benchmark_test_src:   &str = "DO_Benchmark_test.txt";
const benchmark_output:        &str = "testdata/out/speed/benchmark.txt";

pub const n:                    u32 = 1000;
const range:             Range<u32> = 10..18;
const padding:           Range<u32> = 10..18;
const nameLength:             usize = 2;
const gap:                    usize = 10;

pub fn create_test_data() {
    clear_files();

    if cfg!(target_os = "windows") {
        panic!("This is a windows machine - Shame on you!")
    } else {
        println!("\nHOSTNAME: {}",String::from_utf8(Command::new("hostname").output().unwrap().stdout).unwrap());
    }

    println!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, namesize: {} \n\n", &n, &range, &padding, &nameLength);

    println!("## create_test_data");
    FileGenerator::generate_source_file_with(DO_Benchmark_test_pre, n, range, padding, nameLength);

    sleep(time::Duration::from_secs(1));

    if cfg!(target_os = "windows") {
        panic!("This is a windows machine - Shame on you!")
    } else if cfg!(target_os = "macos") {
        Command::new("gshuf")
            .arg(DO_Benchmark_test_pre)
            .arg("-o")
            .arg(DO_Benchmark_test_src)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("shuf")
            .arg(DO_Benchmark_test_pre)
            .arg("-o")
            .arg(DO_Benchmark_test_src)
            .output()
            .expect("failed to execute process")
    };

    sleep(time::Duration::from_secs(1));

    println!("\n## load_to_redblack");
    let mut sw = Stopwatch::start_new();
    load_to_redblack(DO_Benchmark_test_src);
    sw.stop();
    println!("\nredblack load time: {}", sw.elapsed().as_millis());
    sleep(time::Duration::from_secs(1));

    println!("\n## build_tree");
    let mut sw = Stopwatch::start_new();
    load_to_tree(DO_Benchmark_test_src);
    sw.stop();
    println!("\ntree load time: {}", sw.elapsed().as_millis());
    sleep(time::Duration::from_secs(1));

    println!("\n## load_to_table");
    let mut sw = Stopwatch::start_new();
    load_to_table(DO_Benchmark_test_src);
    sw.stop();
    println!("\ntable load time: {}", sw.elapsed().as_millis());
    sleep(time::Duration::from_secs(1));

    println!("\n## search_time_tree");
    let tree_time = search_time(TREE_PAYLOAD,Tree::gen_tree_map, Tree::find_value_on_map);
    sleep(time::Duration::from_secs(1));

    println!("\n## search_time_redblack");
    let redblack_time = search_time(REDBLACK_PAYLOAD, RedBlack::gen_tree_map, RedBlack::find_value_on_map);
    sleep(time::Duration::from_secs(1));

    println!("\n## search_time_table");
    let table_time = search_time(TABLE_PAYLOAD, Table::gen_ip_table, Table::find_value_on_map);
    sleep(time::Duration::from_secs(1));

    println!();
    println!("{}",tree_time);
    println!("{}",redblack_time);
    println!("{}",table_time);

    clear_files();
}

fn clear_files() {
    fs::remove_file(DO_Benchmark_test_pre);
    fs::remove_file(DO_Benchmark_test_src);
    fs::remove_file(TREE_PATH);
    fs::remove_file(TABLE_PATH);
    fs::remove_file(REDBLACK_PATH);
    fs::remove_file(TABLE_PAYLOAD);
    fs::remove_file(TREE_PAYLOAD);
    fs::remove_file(REDBLACK_PAYLOAD);
}

fn search_time(payload_path: &str, structure: fn() -> MmapMut, finder: fn(u32, &MmapMut, &MmapMut) -> Option<String>) -> String {
    let src = DO_Benchmark_test_src;

    let requests = FileGenerator::generate_lookup_testdata(src,gap);
    let length = requests.len();
    assert!(length > 0);

    let structure = structure();
    let name_table = NameTable::gen_name_table_from_path(payload_path);
    let mut numberSkipped = 0;

    let mut i = 0;
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
        if i % (length/100 + 1) == 0 { print!("-"); io::stdout().flush(); }
        i += 1;
    }
    sw.stop();
    format!("Search time --- #{} micro seconds, #{} of requests ran, #{} skipped", sw.elapsed().as_micros(), length, numberSkipped)
}