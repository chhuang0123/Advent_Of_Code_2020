use std::collections::HashMap;

#[test]
fn test_validate_questions_success() {
    assert_eq!(validate_questions(String::from("abac")), 3);
    assert_eq!(validate_questions(String::from("aca")), 2);
    assert_eq!(validate_questions(String::from("aaa")), 1);
}

pub fn validate_questions(_source: String) -> i32 {
    let mut questions: Vec<char> = Vec::new();
    for token in _source.chars() {
        if (token as u8 >= 97 && token as u8 <= 122) && !questions.contains(&token) {
            questions.push(token);
        }
    }

    questions.len() as i32
}

#[test]
fn test_validate_questions_for_all_success() {
    assert_eq!(validate_questions_for_all(String::from("abc")), 3);
    assert_eq!(validate_questions_for_all(String::from("a b c")), 0);
    assert_eq!(validate_questions_for_all(String::from("ab ac")), 1);
    assert_eq!(validate_questions_for_all(String::from("a a a a")), 1);
    assert_eq!(validate_questions_for_all(String::from("b")), 1);
}

pub fn validate_questions_for_all(_source: String) -> usize {
    let people_count: usize = _source.split_ascii_whitespace().count();

    let mut questions: HashMap<char, i32> = HashMap::new();
    for token in _source.chars() {
        if token as u8 >= 97 && token as u8 <= 122 {
            let counter = questions.entry(token).or_insert(0);
            *counter += 1;
        }
    }

    questions
        .iter()
        .filter(|&(_, v)| *v as usize == people_count)
        .count()
}
