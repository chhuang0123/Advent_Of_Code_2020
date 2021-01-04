pub fn is_valid(target_index: usize, number_count: usize, numbers: &[u64]) -> bool {
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
