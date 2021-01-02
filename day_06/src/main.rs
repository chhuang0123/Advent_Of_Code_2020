#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use day_06::{validate_questions, validate_questions_for_all};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut valid_count = 0;
    let mut all_info_line: String = String::new();
    for (_, line) in reader.lines().enumerate() {
        let tmp_line: String = line.unwrap();
        if tmp_line.is_empty() {
            valid_count += validate_questions(String::from(&all_info_line));
            all_info_line.clear();
        } else {
            all_info_line.push_str(&tmp_line.to_string());
        }
    }

    valid_count += validate_questions(String::from(&all_info_line));
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
            valid_count += validate_questions_for_all(String::from(&all_info_line));
            all_info_line.clear();
        } else {
            all_info_line.push_str(&format!(" {}", &tmp_line));
        }
    }

    valid_count += validate_questions_for_all(String::from(&all_info_line));
    println!("part 2: {} valid", valid_count);
}
