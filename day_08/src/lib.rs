#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::neg_multiply
)]

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_success() {
        assert_eq!(parse_rule(String::from("nop +0")), (1, 0));
        assert_eq!(parse_rule(String::from("acc +1")), (1, 1));
        assert_eq!(parse_rule(String::from("jmp +4")), (4, 0));
        assert_eq!(parse_rule(String::from("acc -2")), (1, -2));
    }
}

fn parse_rule(rule: String) -> (i32, i32) {
    let re = Regex::new(r"([a-z]{3})\s(\S{1})(\d+)").unwrap();

    let mut result = (0, 0);
    for tokens in re.captures_iter(&rule) {
        if &tokens[1] == "jmp" {
            let move_value: i32 = tokens[3].parse().unwrap();
            if &tokens[2] == "+" {
                result.0 = move_value;
            } else {
                result.0 = move_value * -1;
            }
        } else {
            result.0 = 1;
        }

        if &tokens[1] == "acc" {
            let acc_value: i32 = tokens[3].parse().unwrap();
            if &tokens[2] == "+" {
                result.1 = acc_value;
            } else {
                result.1 = acc_value * -1;
            }
        }
    }

    result
}

pub fn get_acc(rules: Vec<String>) -> (i32, i32) {
    let mut acc_value: i32 = 0;
    let mut index: i32 = 0;
    let mut indices: Vec<i32> = vec![0];
    let max = rules.len() as i32;

    loop {
        let rule = rules.get(index as usize).unwrap();
        let result = parse_rule(rule.clone());
        index += result.0;
        acc_value += result.1;

        if index < 0 || index > max {
            break;
        }

        if index == max {
            return (acc_value, max);
        }

        if indices.contains(&index) {
            break;
        }
        indices.push(index);
    }

    (acc_value, index)
}
