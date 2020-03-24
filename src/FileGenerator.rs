use rand::Rng;
use rand::distributions::Alphanumeric;
use std::io::{LineWriter, Write, BufRead, Error};
use std::fs::File;
use std::ops::Range;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::min;
use std::fs;
use crate::{SP_100_000, thisFileWillBeDeleted, get_buffer, Utils, Entry, SP_500_000, SP_5_000_000, SP_50_000, MAP_PATH, load_to_tree, Tree, SP_10_000, SP_1_000_000, load_to_table, IP_TABLE_1_000_000, NAME_TABLE_1_000_000, load_to_table_on_path, TREE_MAP_1_000_000, TREE_MAP_500_000};
use regex::bytes::Regex;

fn generate_random_ip_firm(rng: &mut ThreadRng) -> String {
    let ip1 : u8 = rng.gen();
    let ip2 : u8 = rng.gen();
    let ip3 : u8 = rng.gen();
    let ip4 : u8 = rng.gen();
    let mut r = String::new();
    r.push_str(&format!("{}",ip1)); r.push('.');
    r.push_str(&format!("{}",ip2)); r.push('.');
    r.push_str(&format!("{}",ip3)); r.push('.');
    r.push_str(&format!("{}",ip4 >> 1)); r.push(' ');
    r.push_str(&format!("{}",ip1)); r.push('.');
    r.push_str(&format!("{}",ip2)); r.push('.');
    r.push_str(&format!("{}",ip3)); r.push('.');
    r.push_str(&format!("{}",ip4)); r.push(' ');
    let name = gen_firm(& rng, 4);
    r.push_str(&name); r.push_str("\n");
    r
}

pub fn generate_source_file(n: usize, s:&str) {
    let file = File::create(s).unwrap();
    let mut file = LineWriter::new(file);
    let mut rng = rand::thread_rng();
    for i in 0..n {
        if i % 1000 == 0 { println!("number of lines created: {}", i); }
        let s = generate_random_ip_firm(&mut rng);
        file.write_all( s.as_bytes());
        file.flush();
    }
}

fn gen_firm(rng: &ThreadRng, size: usize) -> String {
    rng.sample_iter(&Alphanumeric).take(size).collect::<String>()
}

pub fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

pub fn generate_source_file_with(s:&str, n: u32, range: Range<u32>, padding: Range<u32>, name_size: usize) {
    let mut rng = thread_rng();
    //println!("generate_source_file_with");

    let mut ip_curser: u32 = 0;

    let file = File::create(s).unwrap();
    let mut file = LineWriter::new(file);

    let mut s = String::new();

    for i in 0..n {
        let r: u32 = if range.start == range.end { range.start } else { rng.gen_range(range.start, range.end) };
        let p: u32 = if padding.start == padding.end { range.start } else { rng.gen_range(padding.start, padding.end) };
        let min_ip: u32 = ip_curser;
        if std::u32::MAX - r < min_ip { println!("broke after {} iterations", i); break; }
        let max_ip: u32 = min_ip + r;
        if std::u32::MAX - p < max_ip { println!("broke after {} iterations", i); break; }
        ip_curser = max_ip + p;

        if std::u32::MAX < max_ip { println!("broke after {} iterations", i); break; }

        //if i % 500_000 == 0 { println!("lines generated: {}", i)}
        //if i % (n/10) == 0 { println!("10% done")}

        let name = gen_firm(& rng, name_size);

        let min_ip: [u8; 4] = transform_u32_to_array_of_u8( min_ip);
        let max_ip: [u8; 4] = transform_u32_to_array_of_u8(max_ip);

        for i in 0..4 {
            s.push_str(&format!("{}",min_ip[i]));
            if i < 3 { s.push('.'); }
        }
        s.push(' ');
        for i in 0..4 {
            s.push_str(&format!("{}",max_ip[i]));
            if i < 3 { s.push('.'); }
        }
        s.push(' ');
        s.push_str(name.as_str());
        s.push_str("\n");

        if i % (n/10) == 0 { println!("Done: {:.2}%", i as f32/n as f32); }

        if i % 100 == 0 {
            file.write_all(s.as_bytes());
            s = String::new();
        }
    }
    file.write_all(s.as_bytes());

    println!("highest ip: {}", ip_curser);
    println!("writing to file - done");
}

pub fn generate_source_file_with_in_mem(s:&str, n: u32, range: Range<u32>, padding: Range<u32>, name_size: usize) {
    let mut rng = thread_rng();
    //println!("generate_source_file_with");

    let mut vec : Vec<(u32,u32,String)> = Vec::new();
    let mut ip_curser: u32 = 0;

    for i in 0..n {
        let r: u32 = if range.start == range.end { range.start } else { rng.gen_range(range.start, range.end) };
        let p: u32 = if padding.start == padding.end { range.start } else { rng.gen_range(padding.start, padding.end) };
        let min_ip: u32 = ip_curser;
        if std::u32::MAX - r < min_ip { println!("broke after {} iterations", i); break; }
        let max_ip: u32 = min_ip + r;
        if std::u32::MAX - p < max_ip { println!("broke after {} iterations", i); break; }
        ip_curser = max_ip + p;

        if std::u32::MAX < max_ip { println!("broke after {} iterations", i); break; }

        //if i % 500_000 == 0 { println!("lines generated: {}", i)}

        let name = gen_firm(& rng, name_size);

        vec.push((min_ip,max_ip,name));
    }

    println!("highest ip: {}", ip_curser);
    //println!("shuffle start");
    vec.shuffle(&mut rng);
    //println!("writing to file");
    let file = File::create(s).unwrap();
    let mut file = LineWriter::new(file);

    let mut counter = 0;
    for (min, max, name) in vec.into_iter() {
        let min_ip: [u8; 4] = transform_u32_to_array_of_u8( min);
        let max_ip: [u8; 4] = transform_u32_to_array_of_u8(max);
        let mut r = String::new();

        for i in 0..4 {
            r.push_str(&format!("{}",min_ip[i]));
            if i < 3 { r.push('.'); }
        }
        r.push(' ');
        for i in 0..4 {
            r.push_str(&format!("{}",max_ip[i]));
            if i < 3 { r.push('.'); }
        }
        r.push(' ');
        r.push_str(name.as_str());
        r.push_str("\n");

        file.write_all(r.as_bytes());
        file.flush();

        //if counter % 500_000 == 0 { println!("lines written: {}", counter)}
        counter += 1;
    }
    //println!("writing to file - done");
}

//#[test]
fn gen_input_file() {
    let src = "testtesttesttest";
    fs::remove_file(src);
    generate_source_file_with(src, 1000,2..2,2..2, 4);
}

//#[test]
fn gen_tree() {
    let src = SP_1_000_000;
    load_to_tree(src, TREE_MAP_1_000_000, Tree::insert_entry);
}

//#[test]
fn gen_table() {
    let src = SP_1_000_000;
    load_to_table_on_path(src, IP_TABLE_1_000_000, NAME_TABLE_1_000_000);
}

//#[test]
pub fn generate_lookup_testdata(src: &str, gap: usize) -> Vec<(u32,String)>{

    let ip_regex = Regex::new(r"(\d{1,3}[.]){3}(\d{1,3})").unwrap();
    let name_regex = Regex::new(r"\b(([A-z]|\d)+\s?)+\b").unwrap();

    let mut vec : Vec<(u32,String)> = Vec::new();
    let mut rng = thread_rng();

    let mut lines = get_buffer(src).lines();
    while let ost = lines.nth(gap) {
        if ost.is_none() { break; }
        let ost= ost.unwrap();
        if ost.is_err() { continue }
        let ost = ost.unwrap();
        if ost.is_empty() { continue; }
        let entry = Utils::get_entry_for_line(&ip_regex, &name_regex, &ost);
        if entry.is_none() { continue }
        let entry = entry.unwrap();
        vec.push((rng.gen_range(entry.min_ip,entry.max_ip), entry.name));
    }

    vec.shuffle(&mut rng);
    vec
}