use stopwatch::Stopwatch;
use crate::{FileGenerator, TREE_PRINT_PATH, MAP_PATH, load_to_tree, load_to_table, Utils, NAME_TABLE, IP_TABLE, u32Size, SP_100_000, SP_10_000, thisFileWillBeDeleted, Table, SP_1_000_000, SP_500_000, SP_50_000, IP_TABLE_1_000_000, NAME_TABLE_1_000_000, TREE_MAP_1_000_000};
use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use crate::Tree;
use crate::Tree::TreePrinter;
use crate::Utils::get_memmap;

#[test]
fn build_time_tree() {
    let src = thisFileWillBeDeleted;

    FileGenerator::generate_source_file_with(src, 10,1..2,0..1, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src, MAP_PATH, Tree::insert_entry);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..10000,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src, MAP_PATH, Tree::insert_entry);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);

    FileGenerator::generate_source_file_with(src, 10000,1..2,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_tree(src, MAP_PATH, Tree::insert_entry);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);
}

//#[ignore]
#[test]
fn build_time_table() {
    let src = thisFileWillBeDeleted;

    FileGenerator::generate_source_file_with(src, 10,1..2,0..1, 4);
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

    FileGenerator::generate_source_file_with(src, 10000,1..2,99..100, 4);
    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
    fs::remove_file(src);
}

#[test]
#[ignore]
fn build_time_tree_vs_table() {
    println!("## build_time_tree_vs_table");
    let src = SP_10_000;

    let mut sw = Stopwatch::start_new();
    load_to_tree(src, MAP_PATH, Tree::insert_entry);
    sw.stop();
    println!("tree score:  {}", sw.elapsed().as_millis());

    let mut sw = Stopwatch::start_new();
    load_to_table(src);
    sw.stop();
    println!("table score: {}", sw.elapsed().as_millis());
}

const PATH_SPEED_TEST_2:         &str = "testdata/out/speed/speed_test_2.txt";
#[ignore]
#[test]
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
                load_to_tree(in_src, map_src, Tree::insert_entry);
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
fn search_time_tree_vs_table() {
    println!("## search_time_tree_vs_table");
    let src = SP_10_000;
    fs::remove_file(IP_TABLE);
    fs::remove_file(NAME_TABLE);
    fs::remove_file(MAP_PATH);

    load_to_table(src);
    let requests = FileGenerator::generate_lookup_testdata(src,50);

    let lookup_table = Table::gen_lookup_table();
    let ip_table = Table::gen_ip_table();

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Table::find_value_on_map(ip, &lookup_table, &ip_table);
        assert!(value.is_some());
        let value = value.unwrap();
        if name != value {
            println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
        }
        //assert_eq!(name, value)
    }
    sw.stop();
    println!("table score: {}", sw.elapsed().as_micros());

    load_to_tree(src, MAP_PATH, Tree::insert_entry);
    let requests = FileGenerator::generate_lookup_testdata(src,50);

    let mmap = Tree::gen_tree_map();

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Tree::find_value_on_map(ip,&mmap);
        assert!(value.is_some());
        let value = value.unwrap();
        if name != value {
            println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
        }
        //assert_eq!(name, value)
    }
    sw.stop();
    println!("tree score:  {}", sw.elapsed().as_micros());
}

#[test]
fn search_time_tree_vs_table_no_file_gen() {
    println!("## search_time_tree_vs_table_no_file_gen");
    let src = SP_1_000_000;

    let requests = FileGenerator::generate_lookup_testdata(src,100);
    let requests2 = requests.clone();
    let lenght = requests.len();

    // ------------------------------------------------------

    let lookup_table = Table::gen_lookup_table_from_path(NAME_TABLE_1_000_000);
    let ip_table = Table::gen_ip_table_from_path(IP_TABLE_1_000_000);

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests {
        let value = Table::find_value_on_map(ip, &lookup_table, &ip_table);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
            }
        } else { println!("Found non - real name: {} - ip: {}", name, ip) }
    }
    sw.stop();
    println!("--- table score: {}, #{} of requests ran", sw.elapsed().as_micros(), lenght);

    // ------------------------------------------------------

    let mmap = Tree::gen_tree_map_on_path(TREE_MAP_1_000_000);

    let mut sw = Stopwatch::start_new();
    for (ip, name) in requests2 {
        let value = Tree::find_value_on_map(ip, &mmap);
        if value.is_some() {
            let value = value.unwrap();
            if name != value {
                println!("Wrong match - real name: {} - found name: {} - ip: {}", name, value, ip);
            }
        } else { println!("Found non - real name: {} - ip: {}", name, ip) }

    }
    sw.stop();
    println!("--- Tree score : {}, #{} of requests ran", sw.elapsed().as_micros(), lenght);

}

#[test]
#[ignore]
fn speed_test_5() {
    let src = SP_100_000;

    let ip = Utils::get_u32_for_ip("0.0.0.0").unwrap();

    let mut sw = Stopwatch::start_new();
    let value = Tree::find_value(ip).unwrap();
    //std::str::from_utf8(&value).expect("bad formatting");
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());

    let lookup_table = Table::gen_lookup_table();
    let ip_table = Table::gen_ip_table();

    let mut sw = Stopwatch::start_new();

    let ipLookup: usize = ip as usize * u32Size;
    let id: u32 = unsafe { *Utils::bytes_to_type(&ip_table[ipLookup..(ipLookup + u32Size)]) };
    //let name = Table::get_name(&lookup_table, id as usize);
    //println!("name: {}", name);
    sw.stop();
    println!("score: {}", sw.elapsed().as_millis());
}

#[test]
#[ignore]
fn test_print_tree_to_file() {
    let src = thisFileWillBeDeleted;
    FileGenerator::generate_source_file_with(src, 100,1..2,99..100, 4);
    load_to_tree(src, MAP_PATH, Tree::insert_entry);
    TreePrinter::print_tree_to_file(TREE_PRINT_PATH);
    fs::remove_file(src);
}