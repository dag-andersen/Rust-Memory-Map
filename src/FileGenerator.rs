

use rand::prelude::ThreadRng;
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::io::{LineWriter, Write};
use std::fs::File;

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
    let name = rng.sample_iter(&Alphanumeric).take(4).collect::<String>();
    r.push_str(&name); r.push_str("\n");
    r
}

pub fn generate_source_file(n: usize, s:&str) {
    let file = File::create(s).unwrap();
    let mut file = LineWriter::new(file);
    for i in 0..n {
        if i % 1000 == 0 { println!("number of lines created: {}", i); }
        let mut rng = rand::thread_rng();
        let s = generate_random_ip_firm(&mut rng);
        file.write_all( s.as_bytes());
        file.flush();
    }
}