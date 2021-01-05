pub fn is_valid(number_count: usize, target_index: usize, numbers: &[u64]) -> bool {
    let target_sum = numbers[target_index];

    for i in (target_index - number_count)..target_index {
        for j in (target_index - number_count + 1)..target_index {
            if numbers[i] + numbers[j] == target_sum {
                // println!(
                //     "{} ({}) + {} ({}) = {} ({})",
                //     numbers[i], i, numbers[j], j, target_sum, target_index
                // );
                return true;
            }
        }
    }

    false
}

pub fn get_long_sum(number_count: usize, invalid_index: usize, numbers: &[u64]) -> Vec<u64> {
    let invalid_number = numbers[invalid_index];

    for i in 0..(invalid_index - number_count + 1) {
        let mut result_numbers: Vec<u64> = Vec::new();
        let mut tmp_sum: u64 = 0;
        for j in 0..number_count {
            result_numbers.push(numbers[i + j]);
            tmp_sum += numbers[i + j];
        }

        if tmp_sum == invalid_number {
            return result_numbers;
        }
        if tmp_sum >= invalid_number {
            break;
        }
    }

    vec![]
}
