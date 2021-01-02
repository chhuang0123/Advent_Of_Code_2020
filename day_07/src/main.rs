use day_07::{match_rule, match_rule_v2, parse_rule};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // println!("{:?}", line);

        let rule = parse_rule(line);
        // println!("{:?}", rule);

        rules.insert(rule.0, rule.1);
    }

    let bags = rules.keys();
    let mut total_valid = 0;
    for key in bags {
        total_valid += match_rule(String::from(key), rules.clone());
    }
    println!("part 1: {} total valid", total_valid);

    // part 2
    let result = match_rule_v2(String::from("shiny gold"), rules.clone());
    println!(
        "part 2: a single shiny gold bag must contain {} other bags.",
        result
    );
}
