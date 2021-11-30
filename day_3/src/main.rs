use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let buf_reader = BufReader::new(file);

    for l in buf_reader.lines() {
        let line = l.unwrap();
        let mut map_line = Vec::new();
        for chr in line.chars() {
            map_line.push(chr);
        }
        map.push(map_line);
    }

    println!("Part 1: {}", slope(3, 1, &map));
    println!("Part 2: {}", slope(1, 1, &map) * slope(3, 1, &map) * slope(5, 1, &map) * slope(7, 1, &map) * slope(1, 2, &map));
}

fn slope(right: usize, down: usize, map: &Vec<Vec<char>>) -> i32{
    let max_down = map.len() - 1;
    let max_right = map[0].len() - 1;
    let mut pos_right = right;
    let mut pos_down = down;
    let mut count = 0;
    while max_down >= pos_down {
        if pos_right > max_right {
            pos_right -= max_right + 1;
        }
        if map[pos_down][pos_right] == '#' {
            count += 1;
        }
        pos_right += right;
        pos_down += down;
    }
    println!("{}", count);
    return count;
}
