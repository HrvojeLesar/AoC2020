use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {

	let start = Instant::now();
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut part1 = 0;
    let mut seat_id_list = Vec::new();
    for l in reader.lines() {
        let line: String = l.unwrap();
        let (fb, lr) = line.split_at(7);
        let id = find_seat(fb, 'F', 127) * 8 + find_seat(lr, 'L', 7);
        seat_id_list.push(id);
        if id > part1 {
            part1 = id;
        }
    }
    println!("Part 1: {}", part1);

    seat_id_list.sort();
    for i in 0..(seat_id_list.len() - 1 as usize) {
        if seat_id_list[i + 1] - seat_id_list[i] > 1 {
            println!("Part 2: {}", seat_id_list[i] + 1);
        }
    }
	
	println!("{:?}", start.elapsed());
}

fn find_seat(directions: &str, lower_half_char: char, mut max_range: u32) -> u32 {
    let mut min_range: u32 = 0;
    directions.chars().for_each(|chr| {
        if chr == lower_half_char {
            max_range = get_range_half(&min_range, &max_range);
        } else {
            min_range = get_range_half(&min_range, &max_range) + 1;
        }
    });
    return min_range;
}


fn get_range_half(min: &u32, max: &u32) -> u32 {
    min + (max - min) / 2
}


#[test]
fn range_test() {
    // lower
    assert_eq!(get_range_half(&0, &127), 63);
    assert_eq!(get_range_half(&32, &63), 47);

    // upper
    assert_eq!(get_range_half(&0, &63) + 1, 32);
    assert_eq!(get_range_half(&40, &47) + 1, 44);
}