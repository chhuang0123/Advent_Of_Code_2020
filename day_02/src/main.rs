#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_valid(source: String) -> i32 {
    // println!("{}", source);

    let index_1 = source.find('-').unwrap();
    let index_2 = source.find(' ').unwrap();
    let index_3 = source.find(':').unwrap();
    let index_4 = source.rfind(' ').unwrap();
    // println!("{} {} {} {}", index_1, index_2, index_3, index_4);

    let min: usize = source.get(0..index_1).unwrap().parse().unwrap();
    let max: usize = source.get(index_1 + 1..index_2).unwrap().parse().unwrap();
    let alphabet = source.get(index_2 + 1..index_3).unwrap();
    let password = source.get(index_4 + 1..source.len()).unwrap();
    // println!("{} {} {} {}", min, max, alphabet, password);

    let count = password.matches(&alphabet).count();
    // println!("{}: {}", alphabet, count);

    if count >= min && count <= max {
        1
    } else {
        0
    }
}

fn is_valid_part_2(source: String) -> i32 {
    // println!("{}", source);

    let index_1 = source.find('-').unwrap();
    let index_2 = source.find(' ').unwrap();
    let index_3 = source.find(':').unwrap();
    let index_4 = source.rfind(' ').unwrap();
    // println!("{} {} {} {}", index_1, index_2, index_3, index_4);

    let alphabet = source.get(index_2 + 1..index_3).unwrap();
    let password = source.get(index_4 + 1..source.len()).unwrap();
    // println!("{} {}", alphabet, password);

    let min: usize = source.get(0..index_1).unwrap().parse().unwrap();
    let max: usize = source.get(index_1 + 1..index_2).unwrap().parse().unwrap();
    let alphabet = source.get(index_2 + 1..index_3).unwrap();
    let password = source.get(index_4 + 1..source.len()).unwrap();
    // println!("{} {} {} {}", min, max, alphabet, password);

    let first = password.get(min - 1..min).unwrap();
    let second = password.get(max - 1..max).unwrap();
    // println!("{} {}", first, second);

    if (first == alphabet && second != alphabet) || (second == alphabet && first != alphabet) {
        1
    } else {
        0
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for (_, line) in reader.lines().enumerate() {
        total += is_valid(line.unwrap());
    }

    println!("part 1: total valid: {}", total);

    // part 2
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    total = 0;
    for (index, line) in reader.lines().enumerate() {
        total += is_valid_part_2(line.unwrap());
    }

    println!("part 2: total valid: {}", total);
}
