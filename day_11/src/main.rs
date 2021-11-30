use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {

    let start = Instant::now();

    let mut seat_layout: Vec<Vec<char>> = Vec::new();
    for l in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line: String = l.unwrap();
        let row: Vec<char> = line.chars().collect();
        seat_layout.push(row);
    }

    let mut next_seat_layout: Vec<Vec<char>> = Vec::new();

    let mut seat_layout_p2: Vec<Vec<char>> = seat_layout.clone();
    let mut next_seat_layout_p2: Vec<Vec<char>> = Vec::new();

    loop {
        next_seat_layout = seat_layout.clone();
        for column in 0..seat_layout.len() {
            for row in 0..seat_layout[0].len() {
                if seat_layout[column][row] == '.' {
                    continue;
                }
                let adj = adjacent_num(&seat_layout, &row, &column);
                match seat_layout[column][row] {
                    'L' => {
                        if adj == 0 {
                            next_seat_layout[column][row] = '#';
                        }
                    },
                    '#' => {
                        if adj >= 4 {
                            next_seat_layout[column][row] = 'L';
                        }
                    },
                    chr => next_seat_layout[column][row] = chr,
                }
            }
        }
        if seat_layout == next_seat_layout {
            break;
        } else {
            seat_layout = next_seat_layout.clone();
        }
    }



    let mut i = 0;
    loop {
        i += 1;
        next_seat_layout_p2 = seat_layout_p2.clone();
        for row in 0..seat_layout_p2.len() {
            for column in 0..seat_layout_p2[0].len() {
                if seat_layout_p2[row][column] == '.' {
                    continue;
                }
                let adj = seen_seets(&seat_layout_p2, &row, &column);
                match seat_layout_p2[row][column] {
                    'L' => {
                        if adj == 0 {
                            next_seat_layout_p2[row][column] = '#';
                        }
                    },
                    '#' => {
                        if adj >= 5 {
                            next_seat_layout_p2[row][column] = 'L';
                        }
                    },
                    chr => next_seat_layout_p2[row][column] = chr,
                }
            }
        }
        if seat_layout_p2 == next_seat_layout_p2 {
            break;
        } else {
            seat_layout_p2 = next_seat_layout_p2.clone();
        }
    }

    let mut seat_count = 0;
    next_seat_layout.iter().for_each(|col| col.iter().for_each(|chr| if *chr == '#' { seat_count += 1}));

    let mut seat_count_p2 = 0;
    next_seat_layout_p2.iter().for_each(|col| col.iter().for_each(|chr| if *chr == '#' { seat_count_p2 += 1}));

    println!("Part 1: {}", seat_count);
    println!("Part 1: {}", seat_count_p2);
    println!("Duration: {:?}", start.elapsed());
}

fn adjacent_num(seats: &Vec<Vec<char>>, row: &usize, column: &usize) -> u32 {
    let mut count = 0;
    let mut row_start: i32 = *row as i32 - 1;
    let mut row_end: i32 = *row as i32 + 2 ;
    let mut column_start: i32 = *column as i32 - 1;
    let mut column_end: i32 = *column  as i32 + 2;
    if *row == 0 {
        row_start = *row as i32;
    }
    if *row >= seats[0].len() - 1 {
        row_end = *row as i32 + 1;
    }

    if *column == 0 {
        column_start = *column as i32;
    }
    if *column >= seats.len() - 1 {
        column_end = *column as i32 + 1;
    }

    for c in column_start..column_end {
        for r in row_start..row_end {
            if count >= 4 {
                return count;
            }
            if c == *column as i32 && r == *row as i32 {
                continue;
            }
            if seats[c as usize][r as usize] == '#' {
                count += 1;
            }
        }
    }

    return count;
}

// retarderano kaj ima biti
fn seen_seets(seats: &Vec<Vec<char>>, row: &usize, column: &usize) -> i32 {
    let mut count = 0;
    
    let mut col = 0;
    let mut row_ = 0;
    // <--- P
    if (*column as i32 - 1) < 0 {
        col = 0;
    } else {
        col = *column - 1;
    }
    for c in (0..=col).rev() {
        if c == *column {
            continue;
        }
        if seats[*row][c] == '#' {
            count += 1;
            break;
        } else if seats[*row][c] == 'L' {
            break;
        }
    }

    // P --->
    for c in (*column + 1)..seats[0].len() {
        if seats[*row][c] == '#' {
            count += 1;
            break;
        } else if seats[*row][c] == 'L' {
            break;
        }
    }

    // ^
    // |
    // |
    // P
    if (*row as i32 - 1) < 0 {
        row_ = 0;
    } else {
        row_ = *row - 1;
    }
    for r in (0..=row_).rev() {
        if r == *row {
            continue;
        }
        if seats[r][*column] == '#' {
            count += 1;
            break;
        } else if seats[r][*column] == 'L' {
            break;
        }
    }

    // P
    // |
    // |
    // ˇ
    for r in (*row + 1)..seats.len() {
        if seats[r][*column] == '#' {
            count += 1;
            break;
        } else if seats[r][*column] == 'L' {
            break;
        }
    }

    // ^
    // \
    //  P
    if (*row as i32 - 1) < 0 {
        row_ = 0;
    } else {
        row_ = *row - 1;
    }
    let mut i = 0;
    for r in (0..=row_).rev() {
        i += 1;
        if r == *row {
            continue;
        }
        if *column as i32 - i < 0 {
            break;
        }
        if seats[r][*column - i as usize] == '#' {
            count += 1;
            break;
        } else if seats[r][*column - i as usize] == 'L' {
            break;
        }
    }

    if count >= 5 {
        return count;
    }
    // P
    //  \
    //   ˇ
    let mut i = 0;
    for r in (*row + 1)..seats.len() {
        i += 1;
        if *column as i32 + i >= seats[0].len() as i32 {
            break;
        }
        if seats[r][*column + i as usize] == '#' {
            count += 1;
            break;
        } else if seats[r][*column + i as usize] == 'L' {
            break;
        }
    }

    if count >= 5 {
        return count;
    }

    //    ^
    //  /
    // P
    
    let mut i = 0;
    if (*row as i32 - 1) < 0 {
        row_ = 0;
    } else {
        row_ = *row - 1;
    }
    for r in (0..=row_).rev() {
        i += 1;
        if r == *row {
            continue;
        }
        if *column as i32 + i >= seats[0].len() as i32 {
            break;
        }
        if seats[r][*column + i as usize] == '#' {
            count += 1;
            break;
        } else if seats[r][*column + i as usize] == 'L' {
            break;
        }
    }

    if count >= 5 {
        return count;
    }
    //    P
    //  /
    // ˇ
    let mut i = 0;
    for r in (*row + 1)..seats.len() {
        i += 1;
        if *column as i32 - i < 0 {
            break;
        }
        if seats[r][*column - i as usize] == '#' {
            count += 1;
            break;
        } else if seats[r][*column - i as usize] == 'L' {
            break;
        }
    }

    return count;
}



fn print_layout(layout: &Vec<Vec<char>>) {
    for row in layout {
        for chr in row {
            print!("{}", chr);
        }
        println!("");
    }
}