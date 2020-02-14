use rand::Rng;
use rand::distributions::Alphanumeric;
use std::io::{LineWriter, Write};
use std::fs::File;
use std::ops::Range;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::min;
use std::fs;

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

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

pub fn generate_source_file_with(s:&str, n: u32, range: Range<u32>, padding: Range<u32>, name_size: usize) {
    let mut rng = thread_rng();

    let mut vec : Vec<(u32,u32,String)> = Vec::new();
    let mut ip_curser: u32 = 0;

    for i in 0..n {
        let r: u32 = if range.start == range.end { range.start } else { rng.gen_range(range.start, range.end) };
        let p: u32 = if padding.start == padding.end { range.start } else { rng.gen_range(padding.start, padding.end) };
        let min_ip: u32 = ip_curser;
        let max_ip: u32 = min_ip + r;
        ip_curser = max_ip + p;

        if std::u32::MAX < max_ip { break; }

        let name = gen_firm(& rng, 4);

        vec.push((min_ip,max_ip,name));
    }
    vec.shuffle(&mut rng);

    let file = File::create(s).unwrap();
    let mut file = LineWriter::new(file);

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
    }
}

#[test]
fn test_print_tree_to_file() {
    let src = "thisFileWillBeDeleted";
    generate_source_file_with(src, 10,1..2,99..100, 4);
    fs::remove_file(src);
}