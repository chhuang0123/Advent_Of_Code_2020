use day_08::get_acc;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rules: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        rules.push(line.unwrap());
    }

    let result = get_acc(rules);
    let acc_value: i32 = result.0;
    let index: i32 = result.1;

    println!(
        "part 1: accumulator = {}, stop index = {}",
        acc_value, index
    );

    // part 2
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rules: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        rules.push(line.unwrap());
    }

    let rule_count = rules.len();
    for i in 0..rule_count {
        // println!("{} - before: {:?}", i, rules);

        let mut tmp_rules = rules.clone();
        if let Some(tmp_rule) = tmp_rules.get_mut(i) {
            if tmp_rule.contains("nop") {
                *tmp_rule = tmp_rule.replace("nop", "jmp");
            } else if tmp_rule.contains("jmp") {
                *tmp_rule = tmp_rule.replace("jmp", "nop");
            }
        };

        if tmp_rules == rules {
            continue;
        }

        // println!("{} after: {:?}", i, tmp_rules);

        let result = get_acc(tmp_rules);
        if result.1 == rule_count as i32 {
            println!("part 2: accumulator = {}, stop rule = {}", result.0, i);
            break;
        }
    }
}
