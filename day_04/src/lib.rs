use regex::Regex;

fn is_valid_filed_name(value: &str) -> bool {
    let valid_values = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    valid_values.contains(&value)
}

pub fn is_valid(source: &str) -> i32 {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let mut count = 0;
    let mut contain_cid = false;

    for token in re.captures_iter(&source) {
        if !is_valid_filed_name(&token[1]) {
            return 0;
        }

        if &token[1] == "cid" {
            contain_cid = true;
        }

        count += 1;
    }

    // println!("{}: {}", source, count);

    if count == 8 || (count == 7 && !contain_cid) {
        1
    } else {
        0
    }
}

fn is_valid_byr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 1920 && year <= 2002
}

fn is_valid_iyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 2010 && year <= 2020
}

fn is_valid_eyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 2020 && year <= 2030
}

fn is_valid_hgt(value_string: &str) -> bool {
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

    result
}

fn is_valid_hcl(value_string: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}").unwrap();
    re.is_match(value_string)
}

fn is_valid_ecl(value: &str) -> bool {
    let valid_values = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_values.contains(&value)
}

fn is_valid_pid(value: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(value)
}

pub fn is_valid_part_2(source: &str) -> i32 {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let mut count = 0;
    let mut contain_cid = false;

    for token in re.captures_iter(&source) {
        if !is_valid_filed_name(&token[1]) {
            return 0;
        }

        if &token[1] == "cid" {
            contain_cid = true;
        }

        count += 1;
    }

    // println!("tokens: {}: ({}) ", source, count);

    // too many missing fields
    if count <= 6 {
        return 0;
    }

    // check fields
    for token in re.captures_iter(&source) {
        if !is_valid_filed_name(&token[1]) {
            return 0;
        }

        if &token[1] == "byr" && !is_valid_byr(&token[2]) {
            return 0;
        }
        if &token[1] == "iyr" && !is_valid_iyr(&token[2]) {
            return 0;
        }
        if &token[1] == "eyr" && !is_valid_eyr(&token[2]) {
            return 0;
        }
        if &token[1] == "hgt" && !is_valid_hgt(&token[2]) {
            return 0;
        }
        if &token[1] == "hcl" && !is_valid_hcl(&token[2]) {
            return 0;
        }
        if &token[1] == "ecl" && !is_valid_ecl(&token[2]) {
            return 0;
        }
        if &token[1] == "pid" && !is_valid_pid(&token[2]) {
            return 0;
        }
    }

    if count == 8 || (count == 7 && !contain_cid) {
        1
    } else {
        0
    }
}
