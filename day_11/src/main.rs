use day_11::{check_seat, print_rows};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // part 1
    let filename = &args[0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<char>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut columns: Vec<char> = Vec::new();
        for ch in line.chars() {
            columns.push(ch);
        }
        rows.push(columns.clone());
    }

    print_rows(&rows);

    let row_count = rows.len();
    let column_count = rows[0].len();

    // println!("row: {:?}, column: {:?}", row_count, column_count);

    let mut is_changed = true;
    let mut change_count = 0;
    while is_changed {
        is_changed = false;

        let mut change_set: Vec<(usize, usize, char)> = Vec::new();
        for i in 0..row_count {
            for j in 0..column_count {
                if rows[i][j] == '.' {
                    continue;
                }

                let result = check_seat(i as i8, j as i8, &rows);
                if rows[i][j] == 'L' && result == 0 {
                    change_set.push((i, j, '#'));
                    is_changed = true;
                    continue;
                }

                if rows[i][j] == '#' && result >= 4 {
                    change_set.push((i, j, 'L'));
                    is_changed = true;
                    continue;
                }
            }
        }

        for item in change_set.iter().take(change_set.len()) {
            rows[item.0][item.1] = item.2;
        }

        change_count += 1;
        println!("{:?}", change_count);
        print_rows(&rows);
    }
}
