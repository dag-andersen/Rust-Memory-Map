
use stopwatch::Stopwatch;
use crate::{FileGenerator, TreePrinter, TREE_PRINT_PATH, store_scr_on_map};
use std::fs;

#[test]
fn test() {
    assert!(true);
}


#[test]
fn speed_test() {
    let src = "thisFileWillBeDeleted";

    FileGenerator::generate_source_file_with(src, 10,1..2,0..1, 4);
    let mut sw = Stopwatch::start_new();
    store_scr_on_map(src);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    println!("{}",sw.elapsed().as_micros());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..10000,99..100, 4);
    let mut sw = Stopwatch::start_new();
    store_scr_on_map(src);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    println!("{}",sw.elapsed().as_micros());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..2,99..100, 4);
    let mut sw = Stopwatch::start_new();
    store_scr_on_map(src);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    sw.stop();
    println!("{}",sw.elapsed().as_micros());
    fs::remove_file(src);
}

#[test]
fn test_print_tree_to_file() {
    let src = "thisFileWillBeDeleted";
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    store_scr_on_map(src);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}