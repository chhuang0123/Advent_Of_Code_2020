#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use day_04::{is_valid, is_valid_part_2};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    // part 2
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut valid_count = 0;
    let mut all_info_line: String = String::new();
    for (_, line) in reader.lines().enumerate() {
        let tmp_line: String = line.unwrap();
        if tmp_line.is_empty() {
            valid_count += is_valid_part_2(&all_info_line);
            all_info_line.clear();
        } else {
            all_info_line.push_str(&format!(" {}", &tmp_line));
        }
    }
    valid_count += is_valid_part_2(&all_info_line);
    all_info_line.clear();
    println!("part 2: {} valid", valid_count);
}
