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

pub fn parse_rule(rule: String) -> (i32, i32) {
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
