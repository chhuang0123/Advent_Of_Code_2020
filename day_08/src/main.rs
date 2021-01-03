use day_08::parse_rule;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rules: Vec<(String, bool)> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        rules.push((line.unwrap(), false));
    }

    let mut acc_value: i32 = 0;
    let mut index: i32 = 0;
    let mut indices: Vec<i32> = vec![0];
    loop {
        let rule = rules.get(index as usize).unwrap();
        let result = parse_rule(String::from(rule.0.clone()));
        index += result.0;
        acc_value += result.1;

        if indices.contains(&index) {
            break;
        }
        indices.push(index);
    }

    println!("part 1: accumulator = {}", acc_value);
}
