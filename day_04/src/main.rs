// #![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_valid(source: &str) -> i32 {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let count = re.captures_iter(&source).count();
    // println!("{}: {}", source, count);

    if count == 8 {
        return 1;
    } else if count == 7 {
        return match source.find("cid") {
            Some(_) => 0,
            _ => 1,
        };
    }

    0
}

fn check_byr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    let result = year >= 1920 && year <= 2002;
    // println!("check_byr: {}", result);
    result
}

fn check_iyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    let result = year >= 2010 && year <= 2020;
    // println!("check_iyr: {}", result);
    result
}

fn check_eyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    let result = year >= 2020 && year <= 2030;
    // println!("check_eyr: {}", result);
    result
}

fn check_hgt(value_string: &str) -> bool {
    let mut result: bool = true;
    if !value_string.ends_with("cm") && !value_string.ends_with("in") {
        result = false;
    }

    if value_string.ends_with("cm") {
        let index = value_string.find("cm").unwrap();
        let value: i32 = value_string.get(0..index).unwrap().parse().unwrap();
        if value < 150 || value > 193 {
            result = false;
        }
    } else if value_string.ends_with("in") {
        let index = value_string.find("in").unwrap();
        let value: i32 = value_string.get(0..index).unwrap().parse().unwrap();
        if value < 59 || value > 76 {
            result = false;
        }
    } else {
        result = false;
    }

    // println!("check_eyr: {}", result);
    result
}

fn check_hcl(value_string: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}").unwrap();
    let result = re.is_match(value_string);
    // println!("check_hcl: {}", result);
    result
}

fn check_ecl(value: &str) -> bool {
    let valid_values = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let result = valid_values.contains(&value);
    // println!("check_ecl: {}", result);
    result
}

fn check_pid(value: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    let result = re.is_match(value);
    // println!("check_pid: {}", result);
    result
}

fn is_valid_part_2(source: &str) -> i32 {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let count = re.captures_iter(&source).count();
    // println!("tokens: {}: ({}) ", source, count);

    if count <= 6 {
        return 0;
    }

    for token in re.captures_iter(&source) {
        // println!("{}: {}", &token[1], &token[2]);

        if &token[1] == "byr" && !check_byr(&token[2]) {
            return 0;
        } else if &token[1] == "iyr" && !check_iyr(&token[2]) {
            return 0;
        } else if &token[1] == "eyr" && !check_eyr(&token[2]) {
            return 0;
        } else if &token[1] == "hgt" && !check_hgt(&token[2]) {
            return 0;
        } else if &token[1] == "hcl" && !check_hcl(&token[2]) {
            return 0;
        } else if &token[1] == "ecl" && !check_ecl(&token[2]) {
            return 0;
        } else if &token[1] == "pid" && !check_pid(&token[2]) {
            return 0;
        }
    }

    if count == 8 {
        return 1;
    } else if count == 7 {
        return match source.find("cid") {
            Some(_) => 0,
            _ => 1,
        };
    }

    0
}

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
