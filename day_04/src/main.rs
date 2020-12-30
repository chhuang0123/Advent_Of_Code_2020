// #![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_valid(source: &str) -> i32 {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let count = re.captures_iter(&source).count();
    // println!("{}: {}", source, count);

    if count == 8 {
        return 1;
    } else if count == 7 {
        return match source.find("cid") {
            Some(_) => 0,
            _ => 1,
        };
    }

    0
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut valid_count = 0;
    let mut all_info_line: String = String::new();
    for (_, line) in reader.lines().enumerate() {
        let tmp_line: String = line.unwrap();
        if tmp_line.is_empty() {
            valid_count += is_valid(&all_info_line);
            all_info_line.clear();
        } else {
            all_info_line.push_str(&format!(" {}", &tmp_line));
        }
    }
    valid_count += is_valid(&all_info_line);
    all_info_line.clear();
    println!("part 1: {} valid", valid_count);
}
