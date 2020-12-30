#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_tree_count(right_step: usize, down_step: usize, lines: Vec<String>) -> u32 {
    let mut tree_count = 0;

    for (line_num, line) in lines.iter().enumerate() {
        if line_num % down_step != 0 {
            continue;
        }
        // print!("{:03}: {:?} ", line_num, line);

        let target_position = (right_step * line_num) % line.len();
        let alphabet = line.get(target_position..target_position + 1).unwrap();
        // println!(
        //     "({:03} -- % {} -> {:03}: {})",
        //     3 * line_num,
        //     line.len(),
        //     target_position,
        //     alphabet
        // );

        if alphabet == "#" {
            tree_count += 1;
        }
    }

    tree_count as u32
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let tmp_line = line.unwrap();
        lines.push(tmp_line);
    }

    let mut tree_count = get_tree_count(3, 1, lines);
    println!("part 1: tree count: {}", tree_count);

    // part 2
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let tmp_line = line.unwrap();
        lines.push(tmp_line);
    }

    let mut final_answer: u32 = 1;
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for slope in slopes {
        let tree_count = get_tree_count(slope.0, slope.1, lines.clone());
        final_answer *= tree_count;
    }

    println!("part 2: multiplied tree count: {}", final_answer);
}
