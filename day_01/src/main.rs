#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_sum(target_sum: i32, numbers: HashMap<i32, bool>) -> (i32, i32) {
    for (key, _) in numbers.iter() {
        let target_number = target_sum - key;
        if numbers.contains_key(&target_number) {
            return (key.clone(), target_number);
        }
    }

    (0, 0)
}

fn main() {
    let filename = String::from("/tmp/input");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers: HashMap<i32, bool> = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        numbers.insert(line.unwrap().parse::<i32>().unwrap(), true);
    }

    let result = find_sum(2020, numbers);
    println!("{} * {} = {}", result.0, result.1, result.0 * result.1);
}
