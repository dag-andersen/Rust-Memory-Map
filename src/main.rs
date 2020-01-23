use std::fs::File;
use std;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = load_file("testdata/set1.txt");
    let mut map : HashMap<String,String> = HashMap::new();
    interate(file, &mut map);
    //println!("{}",map.get("Netcompany").unwrap().as_str());
}

fn load_file(name: &str) -> BufReader<std::fs::File> {
    println!("load_file Called");
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);
    reader
}

fn interate(reader: BufReader<std::fs::File>, map: &mut HashMap<String,String>) {
    println!("Interate Called");
    for (_, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        let mut iter = l.split_whitespace();
        let v: Vec<&str> = l.split(' ').collect();

        let hej = "he";

        let key = (v[0], v[1]);
        let valueCompany = v[2];
        add_to_map(key, valueCompany, map);
    }
}

fn add_to_map(key: (&str,&str), value: &str, map: &mut HashMap<String, String>) {
    println!("- item added {} {}", key.0, value);
    map.insert(key.0.to_string(), value.to_string());
}

#[test]
fn verify_test() {
    let file = load_file("testdata/set1.txt");
    let mut map : HashMap<String,String> = HashMap::new();
    interate(file, &mut map);

    assert_eq!("Netcompany", map.get("1113333").unwrap());
}
