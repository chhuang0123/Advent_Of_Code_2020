use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_success() {
        assert_eq!(
            parse_rule(String::from(
                "light red bags contain 1 bright white bag, 2 muted yellow bags."
            )),
            (
                String::from("light red"),
                vec![
                    (1, String::from("bright white")),
                    (2, String::from("muted yellow")),
                ],
            )
        );

        assert_eq!(
            parse_rule(String::from("bright white bags contain 1 shiny gold bag.")),
            (
                String::from("bright white"),
                vec![(1, String::from("shiny gold"))],
            )
        );

        assert_eq!(
            parse_rule(String::from("faded blue bags contain no other bags.")),
            (String::from("faded blue"), vec![(0, String::from(""))])
        );
    }
}

fn parse_sub_bag_rule(_source: String) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();

    let re = Regex::new(r"(\d+)\s(\S+\s\S+)\sbag").unwrap();
    for tokens in re.captures_iter(&_source) {
        let sub_bag_count: usize = String::from(&tokens[1]).parse().unwrap();
        let sub_bag_name: String = String::from(&tokens[2]);
        result.push((sub_bag_count, sub_bag_name));
    }

    result
}

pub fn parse_rule(_source: String) -> (String, Vec<(usize, String)>) {
    let rule: Vec<&str> = _source.split(" bags contain ").collect();
    // println!("{:?}", rule);

    let bag = rule[0];
    let sub_bag_text = rule[1];

    if sub_bag_text.contains("no other bag") {
        (String::from(bag), vec![(0, String::from(""))])
    } else {
        (
            String::from(bag),
            parse_sub_bag_rule(String::from(sub_bag_text)),
        )
    }
}

pub fn match_rule(key: String, rules: HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut bag_stack: Vec<String> = vec![key];
    // println!("-> start: {:?}", bag_stack);

    while bag_stack.len() != 0 {
        // println!("current: {:?}", bag_stack);

        let tmp_key: String = bag_stack.pop().unwrap();
        let rules = rules.get(&tmp_key).unwrap();

        // println!("matched rules: {:?}", rules);

        for rule in rules {
            if rule.1 == "shiny gold" {
                return 1;
            }

            if rule.0 == 0 {
                continue;
            }

            // println!("add : {:?}", rule.1);
            bag_stack.push(String::from(rule.1.clone()));
        }
    }

    0
}

pub fn match_rule_v2(key: String, rules: HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut pop_count = 0;
    let mut bag_stack: Vec<String> = vec![key];
    // println!("-> start: {:?}", bag_stack);

    while bag_stack.len() != 0 {
        // println!("current: {:?}", bag_stack);

        pop_count += 1;
        let tmp_key: String = bag_stack.pop().unwrap();
        let rules = rules.get(&tmp_key).unwrap();

        // println!("matched rules: {:?}", rules);

        for rule in rules {
            if rule.0 == 0 {
                continue;
            }

            // println!("add : {:?}", rule.1);
            for _ in 0..rule.0 {
                bag_stack.push(String::from(rule.1.clone()));
            }
        }
    }

    pop_count - 1
}
