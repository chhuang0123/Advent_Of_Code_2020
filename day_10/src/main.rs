use day_10::get_diff_count;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<u64> = Vec::new();
    let mut max_number = 0;
    for (_, line) in reader.lines().enumerate() {
        let tmp_number: u64 = line.unwrap().parse().unwrap();
        if tmp_number > max_number {
            max_number = tmp_number;
        }
        numbers.push(tmp_number);
    }
    let max_number = numbers.iter().max().unwrap() + 3;
    numbers.push(max_number);
    numbers.sort_unstable();

    println!("{:?}", numbers);

    let diff_count = get_diff_count(&numbers);
    println!(
        "part 1: \n{} (diff 1) * {} (diff 3) = {}",
        diff_count.0,
        diff_count.2,
        diff_count.0 * diff_count.2
    );

    // part 2
    numbers.insert(0, 0);
    let mut path_value: HashMap<u64, u64> = HashMap::new();
    let mut path_list: HashMap<u64, Vec<u64>> = HashMap::new();
    for i in 0..numbers.len() {
        let mut tmp_path_list = vec![];

        for j in 1..4 {
            if i + j >= numbers.len() {
                break;
            }

            if numbers[i + j] - numbers[i] <= 3 {
                tmp_path_list.push(numbers[i + j]);
            }
        }
        path_value.insert(numbers[i], 0);
        path_list.insert(numbers[i], tmp_path_list);

        if i == numbers.len() - 1 {
            path_value.insert(numbers[i], 1);
        }
    }

    println!("part 2:");
    // println!("{:?}", path_value);
    // println!("{:?}", path_list);

    numbers.reverse();
    for i in 1..numbers.len() {
        let tmp_value = numbers[i];
        print!("from {:?} to ", tmp_value);

        let tmp_path_list = &path_list[&tmp_value];
        let mut tmp_sum: u64 = 0;

        for j in 0..tmp_path_list.len() {
            print!("{:?} ", tmp_path_list[j]);
            tmp_sum += path_value[&tmp_path_list[j]];
        }

        path_value.insert(tmp_value, tmp_sum);
        println!("is {:?} path(s)", tmp_sum);
    }
}
