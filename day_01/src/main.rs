#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_sum(target_sum: i32, numbers: &HashMap<i32, bool>) -> (i32, i32) {
    for (key, _) in numbers.iter() {
        let target_number = target_sum - key;
        if numbers.contains_key(&target_number) {
            return (key.clone(), target_number);
        }
    }

    (0, 0)
}

fn main() {
    // 2 entries
    let filename_2 = String::from("/tmp/input.2");
    let file = File::open(filename_2).unwrap();
    let reader = BufReader::new(file);
    let mut numbers_2: HashMap<i32, bool> = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        numbers_2.insert(line.unwrap().parse::<i32>().unwrap(), true);
    }

    let result_2 = find_sum(2020, &numbers_2);
    if result_2 != (0, 0) {
        println!(
            "{} * {} = {}",
            result_2.0,
            result_2.1,
            result_2.0 * result_2.1
        );
    }

    // 3 entries
    let filename_3 = String::from("/tmp/input.3");
    let file = File::open(filename_3).unwrap();
    let reader = BufReader::new(file);
    let mut numbers_3: HashMap<i32, bool> = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        numbers_3.insert(line.unwrap().parse::<i32>().unwrap(), true);
    }

    let mut result_3 = (0, 0, 0);
    for (number, _) in numbers_3.iter() {
        let max_sum: i32 = 2020 - number;
        let tmp_result = find_sum(max_sum, &numbers_3);
        if tmp_result != (0, 0) {
            result_3 = (number.clone(), tmp_result.0, tmp_result.1);
        }
    }

    if result_3 != (0, 0, 0) {
        println!(
            "{} * {} * {} = {}",
            result_3.0,
            result_3.1,
            result_3.2,
            result_3.0 * result_3.1 * result_3.2
        );
    }
}
