pub fn get_diff_count(numbers: &Vec<u64>) -> (u64, u64, u64) {
    let mut diff_count: (u64, u64, u64) = (0, 0, 0);
    let mut previous_number: u64 = 0;
    for item in numbers.iter().take(numbers.len()) {
        if item - previous_number == 1 {
            diff_count.0 += 1;
        } else if item - previous_number == 2 {
            diff_count.1 += 1;
        } else {
            diff_count.2 += 1;
        }
        previous_number = *item;
    }

    diff_count
}
