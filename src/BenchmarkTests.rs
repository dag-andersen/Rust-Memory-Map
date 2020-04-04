use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, TREE_PATH, load_to_tree_on_path, load_to_table, Utils, TABLE_PATH, u32Size, SP_100_000, SP_10_000, thisFileWillBeDeleted, Table, SP_1_000_000, SP_500_000, SP_50_000, NameTable, load_to_tree, load_to_redblack, RedBlack, REDBLACK_PAYLOAD, TABLE_PAYLOAD, TREE_PAYLOAD};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::Tree;
use crate::Tree::TreePrinter;
use crate::Utils::get_memmap;
use crate::FileGenerator::{generate_source_file_with, generate_source_file_with_in_mem};
use std::ops::Range;

#[test]
fn build_time_tree() {
    println!("## build_time_tree");
    let src = thisFileWillBeDeleted;

    FileGenerator::generate_source_file_with(src, 10,1..1,0..1, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..1,100..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,10000..10000,100..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);
}

//#[ignore]
#[test]
fn build_time_table() {
    println!("## build_time_table");
    let src = thisFileWillBeDeleted;

    FileGenerator::generate_source_file_with(src, 10,1..1,0..1, 4);
    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..1,100..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,10000..10000,100..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);
}

//#[test]
fn build_time_tree_vs_table() {
    println!("## build_time_tree_vs_table");
    let src = SP_100_000;

    let mut sw = Stopwatch::start_new();
    load_to_tree(src);
    sw.stop();
    println!("tree score:  {}", sw.elapsed().as_millis());

    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("table score: {}", sw.elapsed().as_millis());
}

const PATH_SPEED_TEST_2:         &str = "testdata/out/speed/speed_test_2.txt";

//#[test]
fn speed_matrix_tree() {
    let in_src = thisFileWillBeDeleted;
    let out_src = PATH_SPEED_TEST_2;

    let file = File::create(out_src).unwrap();
    let mut writer = LineWriter::new(file);

    const number_of_rows_scale: u32 =   4;
    const range_length_scale:   u32 = 100;
    const padding_length_scale: u32 = 100;

    for number_of_rows in 1..5 {
        writer.write_all(format!("Number of rows: {}\n", number_of_rows_scale.pow(number_of_rows)).as_bytes());
        for range_length in 1..5 {
            writer.write_all(format!("--- range length: {}\n", range_length_scale.pow(range_length)).as_bytes());
            for padding_length in 1..5 {
                writer.write_all(format!("------ padding length: {}\n", padding_length_scale.pow(padding_length)).as_bytes());
                FileGenerator::generate_source_file_with(
                    in_src,
                    number_of_rows_scale.pow(number_of_rows),
                    1..range_length_scale.pow(range_length),
                    1..padding_length_scale.pow(padding_length),
                    4);
                let mut sw = Stopwatch::start_new();
                load_to_tree(in_src);
                TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
                sw.stop();
                writer.write_all(format!("------------------------------ {}",sw.elapsed().as_micros()).as_bytes());
                writer.write_all("\n".as_bytes());
                fs::remove_file(in_src);
            }
            writer.write_all("\n".as_bytes());
        }
        writer.write_all("\n".as_bytes());
    }
}

#[test]
fn search_time_tree_vs_RedBlack_vs_table() {
    println!("## search_time_tree_vs_table");

    pub const n:                    u32 = 1500;
    const range:             Range<u32> = 10..18;
    const padding:           Range<u32> = 10..18;
    const nameLength:             usize = 2;
    const gap:                    usize = 10;

    let src = thisFileWillBeDeleted;
    fs::remove_file(src);
    generate_source_file_with_in_mem(src, n,range, padding, nameLength);
    println!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, namesize: {}, gap: {}\n\n", &n, &range, &padding, &nameLength, &gap);

    let requests1 = FileGenerator::generate_lookup_testdata(src,20);
    let requests2 = requests1.clone();
    let requests3 = requests1.clone();
    let length = requests1.len();
    println!("#{} requests created", length);

    load_to_table(src);
    let name_table = NameTable::gen_name_table_from_path(TABLE_PAYLOAD);
    let ip_table = Table::gen_ip_table();

    let mut counter = 0;

    println!("start searching");
    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests1 {
        let value = Table::find_value_on_map(ip, &ip_table, &name_table);
        assert!(value.is_some());
        let value = value.unwrap();
        //if counter % (length/10) == 0 { println!("Found: {:.2}%", counter as f32/length as f32); }
        //if name != value {
        //    println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
        //}
        assert_eq!(name, value);
        //counter += 1;
    }
    sw.stop();
    let tableTime = sw.elapsed().as_micros();
    println!("--- Table time: {}, #{} of requests ran", tableTime, length);

    counter = 0;
    load_to_tree(src);
    let mmap = Tree::gen_tree_map();
    let name_table = NameTable::gen_name_table_from_path(TREE_PAYLOAD);

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests2 {
        let value = Tree::find_value_on_map(ip, &mmap, &name_table);
        assert!(value.is_some());
        let value = value.unwrap();
        //if counter % (length/10) == 0 { println!("Found: {:.2}%", counter as f32/length as f32); }
        //if name != value {
        //    println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
        //}
        assert_eq!(name, value);
        //counter += 1;
    }
    sw.stop();
    let treeTime = sw.elapsed().as_micros();
    println!("--- Tree time : {}, #{} of requests ran", treeTime, length);

    counter = 0;
    load_to_redblack(src);
    let mmap = RedBlack::gen_tree_map();
    let name_table = NameTable::gen_name_table_from_path(REDBLACK_PAYLOAD);

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests3 {
        let value = RedBlack::find_value_on_map(ip, &mmap, &name_table);
        assert!(value.is_some());
        let value = value.unwrap();
        //if counter % (length/10) == 0 { println!("Found: {:.2}%", counter as f32/length as f32); }
        //if name != value {
        //    println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
        //}
        assert_eq!(name, value);
        //counter += 1;
    }
    sw.stop();
    let treeTime = sw.elapsed().as_micros();
    println!("--- ReadBlack time : {}, #{} of requests ran", treeTime, length);
    //assert!(tableTime < treeTime)
}

#[test]
#[ignore]
fn test_print_tree_to_file() {
    let src = thisFileWillBeDeleted;
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    load_to_tree(src);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}

#[test]
#[ignore]
fn genfile() {
    let src = "genfile";
    fs::remove_file(src);
    generate_source_file_with(src, 10_000_000,1..100,0..100, 4);
}
