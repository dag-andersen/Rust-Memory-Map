use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, TREE_PATH, Utils, TABLE_PATH, u32Size, test_set_5, thisFileWillBeDeleted, Table, test_set_6, PayloadMap, RedBlack, REDBLACK_PAYLOAD, TABLE_PAYLOAD, TREE_PAYLOAD, test_set_1};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::BST;
use crate::BST::TreePrinter;
use crate::Utils::get_memmap;
use crate::FileGenerator::{generate_source_file, generate_source_file_shuffled};
use std::ops::Range;
use memmap::MmapMut;

#[test]
fn find_hardcoded_node_in_tree() {
    find_hardcoded_node(BST::build, BST::find_value)
}

#[test]
fn find_hardcoded_node_in_redblack() {
    find_hardcoded_node(RedBlack::build, RedBlack::find_value)
}

#[test]
fn find_hardcoded_node_in_table() {
    find_hardcoded_node(Table::build, Table::find_value)
}

fn find_hardcoded_node(loader: fn(&str), finder: fn(u32) -> Option<String>) {
    loader(test_set_1);

    let name = finder(Utils::get_u32_for_ip("000.000.000.015").unwrap());
    assert!(name.is_some());
    assert_eq!(name.unwrap(),"Siteimprove");

    let name = finder(Utils::get_u32_for_ip("000.000.002.015").unwrap());
    assert!(name.is_some());
    assert_eq!(name.unwrap(),"Olesen");

    assert!(finder(Utils::get_u32_for_ip("000.000.000.001").unwrap()).is_none());
    assert!(finder(Utils::get_u32_for_ip("001.000.000.000").unwrap()).is_none());
}

#[test]
fn find_random_gen_requests_in_tree_in_hardcoded_data() {
    find_random_gen_request_in_hardcoded_data(BST::build, BST::find_value);
}

#[test]
fn find_random_gen_requests_in_redblack_in_hardcoded_data() {
    find_random_gen_request_in_hardcoded_data(RedBlack::build, RedBlack::find_value);
}

#[test]
fn find_random_gen_requests_in_table_in_hardcoded_data() {
    find_random_gen_request_in_hardcoded_data(Table::build, Table::find_value);
}

fn find_random_gen_request_in_hardcoded_data(builder: fn(&str), finder: fn(u32) -> Option<String>) {
    let scr = test_set_5;
    builder(scr);
    let requests = FileGenerator::generate_lookup_testdata(scr,50);

    for (ip, name) in requests {
        let value = finder(ip);
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(name, value)
    }
}

#[test]
fn build_and_search_table_with_random_data() {
    build_and_search_datastructure_with_random_data(TABLE_PAYLOAD, Table::build, Table::gen_ip_table, Table::find_value_on_map)
}

#[test]
fn build_and_search_BST_with_random_data() {
    build_and_search_datastructure_with_random_data(TREE_PAYLOAD, BST::build, BST::gen_tree_map, BST::find_value_on_map)
}

#[test]
fn build_and_search_redblack_with_random_data() {
    build_and_search_datastructure_with_random_data(REDBLACK_PAYLOAD, RedBlack::build, RedBlack::gen_tree_map, RedBlack::find_value_on_map)
}

fn build_and_search_datastructure_with_random_data(payload_path: &str, builder: fn(&str), structure: fn() -> MmapMut, finder: fn(u32, &MmapMut, &MmapMut) -> Option<String>) {

    pub const n:                    u32 = 1500;
    const range:             Range<u32> = 10..18;
    const padding:           Range<u32> = 10..18;
    const nameLength:             usize = 2;
    const gap:                    usize = 10;

    let src = thisFileWillBeDeleted;
    fs::remove_file(src);
    generate_source_file_shuffled(src, n, range, padding, nameLength);
    println!("Benchmark input: n: {}, range: {:#?}, padding: {:#?}, payload_size: {}, gap: {}\n\n", &n, &range, &padding, &nameLength, &gap);

    let requests = FileGenerator::generate_lookup_testdata(src, gap);
    let length = requests.len();
    println!("#{} requests created", length);

    builder(src);
    let data_structure = structure();
    let payload_table = PayloadMap::gen_payload_map_from_path(payload_path);

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = finder(ip, &data_structure, &payload_table);
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(name, value);
    }
    sw.stop();
    let treeTime = sw.elapsed().as_micros();
    println!("--- speed: {} microseconds, #{} of requests ran", treeTime, length);
}