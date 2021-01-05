use day_09::{get_long_sum, is_valid};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<u64> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let tmp_number: u64 = line.unwrap().parse::<u64>().unwrap();
        numbers.push(tmp_number);
    }

    // println!("numbers: {:?}", numbers);

    let target_preamble = 25;
    let mut invalid_number: u64 = 0;
    let mut invalid_index: usize = 0;
    for i in target_preamble..numbers.len() {
        if !is_valid(target_preamble, i, &numbers) {
            invalid_number = numbers[i];
            invalid_index = i;
            break;
        }
    }

    println!(
        "part 1: {} ({}) is not a valid {}-number preamble.",
        invalid_number, invalid_index, target_preamble
    );

    // part 2
    println!("part 2: sum of {} are", invalid_number);
    let mut smallest_number: u64 = invalid_number;
    let mut largest_number: u64 = 0;
    for i in 3..target_preamble {
        let result: Vec<u64> = get_long_sum(i, invalid_index, &numbers);
        if !result.is_empty() {
            for item in result.iter().take(result.len()) {
                println!("{}", item);

                if item > &largest_number {
                    largest_number = *item;
                }

                if item < &smallest_number {
                    smallest_number = *item;
                }
            }
        }
    }
    println!("----------");
    println!(
        "smallest number ({}) + largest number ({}) = ({})",
        smallest_number,
        largest_number,
        smallest_number + largest_number
    );
}
