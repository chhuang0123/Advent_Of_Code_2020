use day_09::is_valid;
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
    for i in target_preamble..numbers.len() {
        if !is_valid(i, target_preamble, &numbers) {
            println!(
                "part 1: {} ({}) is not valid {}-number preamble.",
                numbers[i], i, target_preamble
            );
        }
    }
}
