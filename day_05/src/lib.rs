#[test]
fn test_parse_row_success() {
    assert_eq!(parse_row(&"FBFBBFF"), 44);
}

#[allow(dead_code)]
fn parse_row(_source: &str) -> i32 {
    let mut source: String = String::from(_source);
    source = source.replace('B', "1");
    source = source.replace('F', "0");

    i32::from_str_radix(&source, 2).unwrap()
}

#[test]
fn test_parse_column_success() {
    assert_eq!(parse_column(&"RLR"), 5);
}

#[allow(dead_code)]
fn parse_column(_source: &str) -> i32 {
    let mut source: String = String::from(_source);
    source = source.replace('R', "1");
    source = source.replace('L', "0");

    i32::from_str_radix(&source, 2).unwrap()
}

#[test]
fn test_parse_boarding_string_success() {
    assert_eq!(parse_boarding_string(String::from("FBFBBFFRLR")), (44, 5));
}

#[allow(dead_code)]
pub fn parse_boarding_string(source: String) -> (i32, i32) {
    let _row_string = &source.get(0..7).unwrap();
    let _column_string = &source.get(7..10).unwrap();

    (parse_row(_row_string), parse_column(_column_string))
}
