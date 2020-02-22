use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, MAP_PATH, load_to_map, load_to_table, Utils, TABLE2, TABLE1, u32Size, SP_100_000, SP_10_000};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::Tree;
use crate::Tree::TreePrinter;
use crate::Utils::get_memmap;

#[test]
fn speed_test() {
    let src = "thisFileWillBeDeleted";

    FileGenerator::generate_source_file_with(src, 10,1..2,0..1, 4);
    let mut sw = Stopwatch::start_new();
    load_to_map(src,  MAP_PATH, Tree::insert_entry);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..10000,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_map(src, MAP_PATH,Tree::insert_entry);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..2,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_map(src, MAP_PATH,Tree::insert_entry);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);
}

const PATH_SPEED_TEST_2:         &str = "testdata/out/speed/speed_test_2.txt";
#[test]
fn speed_test_2() {
    let in_src = "thisFileWillBeDeleted";
    let out_src = PATH_SPEED_TEST_2;
    let map_src = MAP_PATH;

    let file = File::create(out_src).unwrap();
    let mut writer = LineWriter::new(file);

    const number_of_rows_scale: u32 = 4;
    const range_length_scale: u32 = 100;
    const padding_length_scale: u32 =    100;

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
                load_to_map(in_src, map_src,Tree::insert_entry);
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
fn speed_test_3() {
    let src = SP_10_000;

    //FileGenerator::generate_source_file_with(src, 1000,1..100,0..10, 4);

    let mut sw = Stopwatch::start_new();
    load_to_map(src,  MAP_PATH, Tree::insert_entry);
    sw.stop();
    println!("tree score: {}", sw.elapsed().as_millis());

    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("table score: {}", sw.elapsed().as_millis());
}

#[test]
#[ignore]
fn speed_test_4() {
    let src = "testdata/in/1_000_000.txt";

    let mut sw = Stopwatch::start_new();
    load_to_map(src,  MAP_PATH, Tree::insert_entry);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());

    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
}

#[test]
#[ignore]
fn speed_test_5() {
    let src = "testdata/in/100_000.txt";

    let ip = Utils::get_u32_for_ip("0.0.0.0").unwrap();

    let mut sw = Stopwatch::start_new();
    let value = Tree::find_value(ip).unwrap();
    std::str::from_utf8(&value).expect("bad formatting");
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());

    let mut lookup_table = get_memmap(TABLE1, 4_000_000_000);
    let mut ip_table = get_memmap(TABLE2, 16_000_000_000);

    let mut sw = Stopwatch::start_new();

    let ipLookup: usize = ip as usize * u32Size;
    let id: u32 = unsafe { *Utils::bytes_to_type(&ip_table[ipLookup..(ipLookup + u32Size)]) };
    //let name = Table::get_name(&lookup_table, id as usize);
    //println!("name: {}", name);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
}

#[test]
fn test_print_tree_to_file() {
    let src = "thisFileWillBeDeleted";
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    load_to_map(src, MAP_PATH,Tree::insert_entry);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}