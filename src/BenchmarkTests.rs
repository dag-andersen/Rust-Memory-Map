use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, store_scr_on_map, MAP_PATH};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::Tree::TreePrinter;

#[test]
fn speed_test() {
    let src = "thisFileWillBeDeleted";

    FileGenerator::generate_source_file_with(src, 10,1..2,0..1, 4);
    let mut sw = Stopwatch::start_new();
    store_scr_on_map(src,  MAP_PATH);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..10000,99..100, 4);
    let mut sw = Stopwatch::start_new();
    store_scr_on_map(src, MAP_PATH);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..2,99..100, 4);
    let mut sw = Stopwatch::start_new();
    store_scr_on_map(src, MAP_PATH);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
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

    const numer_of_rows_scale: u32 = 4;
    const range_length_scale: u32 = 100;
    const padding_lenght_scale: u32 = 100;

    for number_of_rows in 1..4 {
        writer.write_all(format!("Number of rows: {}\n", numer_of_rows_scale.pow(number_of_rows)).as_bytes());
        for range_length in 1..4 {
            writer.write_all(format!("--- range length: {}\n", range_length_scale.pow(range_length)).as_bytes());
            for padding_length in 1..4 {
                writer.write_all(format!("------ padding length: {}\n", padding_lenght_scale.pow(padding_length)).as_bytes());
                FileGenerator::generate_source_file_with(
                    in_src,
                    numer_of_rows_scale.pow(number_of_rows),
                    1..range_length_scale.pow(range_length),
                    1..padding_lenght_scale.pow(padding_length),
                    4);
                let mut sw = Stopwatch::start_new();
                store_scr_on_map(in_src, map_src);
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
fn test_print_tree_to_file() {
    let src = "thisFileWillBeDeleted";
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    store_scr_on_map(src, MAP_PATH);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}