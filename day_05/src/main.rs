#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use day_05::parse_boarding_string;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut max_value = 0;
    for (_, line) in reader.lines().enumerate() {
        let row_column = parse_boarding_string(line.unwrap());
        let result = row_column.0 * 8 + row_column.1;
        if result > max_value {
            max_value = result;
        }
    }
    println!("part 1: {} is max value", max_value);

    // part 2
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut min_value: i32 = 10000000;
    let mut max_value: i32 = 0;
    let mut seats: Vec<i32> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let row_column: (i32, i32) = parse_boarding_string(line.unwrap());
        let result: i32 = row_column.0 * 8 + row_column.1;

        seats.push(result);

        if result > max_value {
            max_value = result;
        }

        if result < min_value {
            min_value = result;
        }
    }

    for i in min_value..max_value {
        if seats.contains(&i) {
            continue;
        }

        println!("part 2: {} is my seat id", i);
        break;
    }
}
