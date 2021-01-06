pub fn print_rows(rows: &[Vec<char>]) {
    let mut count = 0;
    for row in rows.iter().take(rows.len()) {
        for column in row.iter().take(row.len()) {
            print!("{}", column);
            if *column == '#' {
                count += 1;
            }
        }
        println!();
    }
    println!("----------> #: {}", count);
}

pub fn check_seat(i: i8, j: i8, rows: &[Vec<char>]) -> i8 {
    let mut occupied_count: i8 = 0;

    let row_count = rows.len();
    let column_count = rows[0].len();

    for x in i - 1..i + 2 {
        for y in j - 1..j + 2 {
            if x < 0
                || y < 0
                || x >= row_count as i8
                || y >= column_count as i8
                || (x == i && y == j)
            {
                continue;
            }

            if rows[x as usize][y as usize] == '.' {
                continue;
            }

            if rows[x as usize][y as usize] == '#' {
                occupied_count += 1;
            }
        }
    }

    occupied_count
}
