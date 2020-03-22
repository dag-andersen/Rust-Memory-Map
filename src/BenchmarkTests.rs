use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, MAP_PATH, load_to_tree, load_to_table, Utils, NAME_TABLE, IP_TABLE, u32Size, SP_100_000, SP_10_000, thisFileWillBeDeleted, Table, SP_1_000_000, SP_500_000, SP_50_000, IP_TABLE_1_000_000, NAME_TABLE_1_000_000, TREE_MAP_1_000_000};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::Tree;
use crate::Tree::TreePrinter;
use crate::Utils::get_memmap;
use crate::FileGenerator::generate_source_file_with;

#[test]
fn build_time_tree() {
    println!("## build_time_tree");
    let src = thisFileWillBeDeleted;

    FileGenerator::generate_source_file_with(src, 10,1..2,0..1, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src, MAP_PATH);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..2,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src, MAP_PATH);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..10000,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src, MAP_PATH);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);
}

//#[ignore]
#[test]
fn build_time_table() {
    println!("## build_time_table");
    let src = thisFileWillBeDeleted;

    FileGenerator::generate_source_file_with(src, 10,1..2,0..1, 4);
    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..2,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..10000,99..100, 4);
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
    load_to_tree(src, MAP_PATH);
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
    let map_src = MAP_PATH;

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
                load_to_tree(in_src, map_src);
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

//#[test]
fn genfile() {
    let src = "genfile";
    fs::remove_file(src);
    generate_source_file_with(src, 10_000_000,1..100,0..100, 4);
}

//#[test]
fn search_time_tree_vs_table() {

    //let src = "genfile";

    println!("## search_time_tree_vs_table");
    let src = SP_100_000;
    fs::remove_file(src);
    generate_source_file_with(src, 100_000,1..1,1_000..1_000, 4);
    fs::remove_file(IP_TABLE);
    fs::remove_file(NAME_TABLE);
    fs::remove_file(MAP_PATH);

    let requests1 = FileGenerator::generate_lookup_testdata(src,5000);
    let requests2 = requests1.clone();
    let length = requests1.len();
    println!("#{} requests created", length);

    load_to_table(src);
    let lookup_table = Table::gen_lookup_table();
    let ip_table = Table::gen_ip_table();

    let mut counter = 0;

    println!("start searching");
    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests1 {
        let value = Table::find_value_on_map(ip, &lookup_table, &ip_table);
        assert!(value.is_some());
        let value = value.unwrap();
        if counter % (length/10) == 0 { println!("Found: {:.2}%", counter as f32/length as f32); }
        if name != value {
            println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
        }
        counter += 1;
    }
    sw.stop();
    println!("--- table score: {}, #{} of requests ran", sw.elapsed().as_micros(), length);

    counter = 0;
    load_to_tree(src, MAP_PATH);
    let mmap = Tree::gen_tree_map();
    let lookup_table = Table::gen_lookup_table();

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests2 {
        let value = Tree::find_value_on_map(ip,&mmap, &lookup_table);
        assert!(value.is_some());
        let value = value.unwrap();
        if counter % (length/10) == 0 { println!("Found: {:.2}%", counter as f32/length as f32); }
        if name != value {
            println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
        }
        counter += 1;
    }
    sw.stop();
    println!("--- tree score : {}, #{} of requests ran", sw.elapsed().as_micros(), length);
}

//#[test]
fn search_time_tree_vs_table_no_file_gen() {
    println!("## search_time_tree_vs_table_no_file_gen");
    let src = SP_1_000_000;

    let requests = FileGenerator::generate_lookup_testdata(src,100);
    let requests2 = requests.clone();
    let length = requests.len();

    // ------------------------------------------------------

    let lookup_table = Table::gen_lookup_table_from_path(NAME_TABLE_1_000_000);
    let ip_table = Table::gen_ip_table_from_path(IP_TABLE_1_000_000);

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Table::find_value_on_map(ip, &lookup_table, &ip_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                println!("Wrong match - real: {} - found: {} - ip: {}", name, value, ip);
            }
        } else { println!("Found none - real name: {} - ip: {}", name, ip) }
    }
    sw.stop();
    println!("--- table score: {}, #{} of requests ran", sw.elapsed().as_micros(), length);

    // ------------------------------------------------------

    let lookup_table = Table::gen_lookup_table_from_path(NAME_TABLE_1_000_000);
    let mmap = Tree::gen_tree_map_on_path(TREE_MAP_1_000_000);

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests2 {
        let value = Tree::find_value_on_map(ip, &mmap, &lookup_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
            }
        } else { println!("Found none - real name: {} - ip: {}", name, ip) }

    }
    sw.stop();
    println!("--- Tree score : {}, #{} of requests ran", sw.elapsed().as_micros(), length);
}

#[test]
#[ignore]
fn test_print_tree_to_file() {
    let src = thisFileWillBeDeleted;
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    load_to_tree(src, MAP_PATH);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}