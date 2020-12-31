use regex::Regex;

fn is_valid_filed_name(value: &str) -> bool {
    let valid_values = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    valid_values.contains(&value)
}

#[test]
fn test_is_valid_filed_name_success() {
    assert_eq!(is_valid_filed_name("byr"), true);
    assert_eq!(is_valid_filed_name("iyr"), true);
    assert_eq!(is_valid_filed_name("eyr"), true);
    assert_eq!(is_valid_filed_name("hgt"), true);
    assert_eq!(is_valid_filed_name("hcl"), true);
    assert_eq!(is_valid_filed_name("ecl"), true);
    assert_eq!(is_valid_filed_name("pid"), true);
    assert_eq!(is_valid_filed_name("cid"), true);
}

#[test]
fn test_is_valid_filed_name_failure() {
    assert_eq!(is_valid_filed_name("abc"), false);
    assert_eq!(is_valid_filed_name(""), false);
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

#[test]
fn test_is_valid_byr_success() {
    assert_eq!(is_valid_byr("1920"), true);
    assert_eq!(is_valid_byr("1921"), true);
    assert_eq!(is_valid_byr("2001"), true);
    assert_eq!(is_valid_byr("2002"), true);
}

#[test]
fn test_is_valid_byr_failure() {
    assert_eq!(is_valid_byr("1919"), false);
    assert_eq!(is_valid_byr("2003"), false);
    assert_eq!(is_valid_byr("-1"), false);
    assert_eq!(is_valid_byr("0"), false);
    assert_eq!(is_valid_byr("10"), false);
    assert_eq!(is_valid_byr("100"), false);
    assert_eq!(is_valid_byr("1000"), false);
    assert_eq!(is_valid_byr("10000"), false);
}

fn is_valid_iyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 2010 && year <= 2020
}

#[test]
fn test_is_valid_iyr_success() {
    assert_eq!(is_valid_iyr("2010"), true);
    assert_eq!(is_valid_iyr("2011"), true);
    assert_eq!(is_valid_iyr("2019"), true);
    assert_eq!(is_valid_iyr("2020"), true);
}

#[test]
fn test_is_valid_iyr_failure() {
    assert_eq!(is_valid_iyr("2009"), false);
    assert_eq!(is_valid_iyr("2021"), false);
    assert_eq!(is_valid_iyr("-1"), false);
    assert_eq!(is_valid_iyr("0"), false);
    assert_eq!(is_valid_iyr("10"), false);
    assert_eq!(is_valid_iyr("100"), false);
    assert_eq!(is_valid_iyr("1000"), false);
    assert_eq!(is_valid_iyr("10000"), false);
}

fn is_valid_eyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 2020 && year <= 2030
}

#[test]
fn test_is_valid_eyr_success() {
    assert_eq!(is_valid_eyr("2020"), true);
    assert_eq!(is_valid_eyr("2021"), true);
    assert_eq!(is_valid_eyr("2029"), true);
    assert_eq!(is_valid_eyr("2030"), true);
}

#[test]
fn test_is_valid_eyr_failure() {
    assert_eq!(is_valid_eyr("2019"), false);
    assert_eq!(is_valid_eyr("2031"), false);
    assert_eq!(is_valid_eyr("-1"), false);
    assert_eq!(is_valid_eyr("0"), false);
    assert_eq!(is_valid_eyr("10"), false);
    assert_eq!(is_valid_eyr("100"), false);
    assert_eq!(is_valid_eyr("1000"), false);
    assert_eq!(is_valid_eyr("10000"), false);
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

#[test]
fn test_is_valid_hgt_success() {
    assert_eq!(is_valid_hgt("150cm"), true);
    assert_eq!(is_valid_hgt("190cm"), true);
    assert_eq!(is_valid_hgt("193cm"), true);

    assert_eq!(is_valid_hgt("59in"), true);
    assert_eq!(is_valid_hgt("70in"), true);
    assert_eq!(is_valid_hgt("76in"), true);
}

#[test]
fn test_is_valid_hgt_failure() {
    assert_eq!(is_valid_hgt("149cm"), false);
    assert_eq!(is_valid_hgt("194cm"), false);

    assert_eq!(is_valid_hgt("58in"), false);
    assert_eq!(is_valid_hgt("77in"), false);
}

fn is_valid_hcl(value_string: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}").unwrap();
    re.is_match(value_string)
}

#[test]
fn test_is_valid_hcl_success() {
    assert_eq!(is_valid_hcl("#123abc"), true);
    assert_eq!(is_valid_hcl("#abc123"), true);
}

#[test]
fn test_is_valid_hcl_failure() {
    assert_eq!(is_valid_hcl("123abc"), false);
    assert_eq!(is_valid_hcl("#123abz"), false);
    assert_eq!(is_valid_hcl("#abz123"), false);
}

fn is_valid_ecl(value: &str) -> bool {
    let valid_values = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_values.contains(&value)
}

#[test]
fn test_is_valid_ecl_success() {
    assert_eq!(is_valid_ecl("amb"), true);
    assert_eq!(is_valid_ecl("blu"), true);
    assert_eq!(is_valid_ecl("brn"), true);
    assert_eq!(is_valid_ecl("gry"), true);
    assert_eq!(is_valid_ecl("grn"), true);
    assert_eq!(is_valid_ecl("hzl"), true);
    assert_eq!(is_valid_ecl("oth"), true);
}

#[test]
fn test_is_valid_ecl_failure() {
    assert_eq!(is_valid_ecl("abc"), false);
}

fn is_valid_pid(value: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(value)
}

#[test]
fn test_is_valid_pid_success() {
    assert_eq!(is_valid_pid("000000000"), true);
    assert_eq!(is_valid_pid("000000001"), true);
    assert_eq!(is_valid_pid("999999999"), true);
}

#[test]
fn test_is_valid_pid_failure() {
    assert_eq!(is_valid_pid("0123456789"), false);
}

pub fn is_valid_part_2(source: &str) -> i32 {
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let count = re.captures_iter(&source).count();

    // println!("tokens: {}: ({}) ", source, count);

    // too many missing fields
    if count <= 6 {
        return 0;
    }

    // check fields
    let mut contain_cid = false;
    for token in re.captures_iter(&source) {
        if !is_valid_filed_name(&token[1]) {
            return 0;
        }

        if &token[1] == "cid" {
            contain_cid = true;
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
