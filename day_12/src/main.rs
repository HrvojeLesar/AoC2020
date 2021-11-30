use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::collections::HashMap;

const DIRECTIONS: [&str; 4] = [
    "N",
    "E",
    "S",
    "W",
];

fn main() {

    let start = Instant::now();

    let instructions: Vec<(String, u32)> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| {
        let line: String = l.unwrap();
        let (action, value) = line.split_at(1);
        (action.to_owned(), value.parse::<u32>().unwrap())
    }).collect();

    let mut positioning = vec![0, 0];
    let mut currently_facing = "E".to_owned();

    for (action, value) in instructions.clone() {
        let mut action = action;
        let mut value: i32 = value as i32;
        let mut index = 0;

        if action == "F" {
            action = currently_facing.clone();
        }

        match action.as_str() {
            "S" => { value = value * -1; },
            "E" => { index = 1; },
            "W" => { index = 1; value = value * -1; },
            "R" | "L" => {
                currently_facing = change_direction(&currently_facing, &action, &(value as u32));
                continue;
                },
            _ => (),
        }

        positioning[index] += value;
    }
    
    println!("Part 1: {}", positioning[0].abs() + positioning[1].abs());


    // N/S | E/W
    let mut waypoint_pos = vec![1, 10];
    let mut ship_pos = vec![0, 0];
    let mut currently_facing = "E".to_owned();

    for (action, value) in instructions {
        let mut action = action;
        let mut value: i32 = value as i32;
        let mut index = 0;

        match action.as_str() {
            "F" => {
                ship_pos[0] += waypoint_pos[0] * value;
                ship_pos[1] += waypoint_pos[1] * value;
                continue;
            }
            "S" => { value = value * -1; },
            "E" => { index = 1; },
            "W" => { index = 1; value = value * -1; },
            "R" | "L" => {
                rotate_waypoint(&mut waypoint_pos, &action, &(value as u32));
                continue;
                },
            _ => (),
        }

        waypoint_pos[index] += value;
    }

    println!("Part 2: {}", ship_pos[0].abs() + ship_pos[1].abs());
    println!("Duration: {:?}", start.elapsed());
}

fn change_direction(cur: &String, dir: &String, deg: &u32) -> String {
    let mut index = get_dir_index(cur);
    let mut increment: i32 = 0;
    match dir.as_str() {
        "R" => increment = 1,
        "L" => increment = -1,
        _ => return "Err".to_owned(),
    }
    let mut turn_times = *deg / 90;
    while turn_times > 0 {
        index = ((((index as i32 + increment) % 4) + 4) % 4) as usize;
        turn_times -= 1;
    }
    return DIRECTIONS[index].to_owned();
}

fn get_dir_index(cur: &String) -> usize {
    for i in 0..4 {
        if DIRECTIONS[i] == *cur {
            return i;
        }
    }
    return 0;
}

fn rotate_waypoint(wp: &mut Vec<i32>, dir: &String, deg: &u32) {
    let mut turn_times = *deg / 90;
    let mut clockwise = -1;
    if dir == "L" {
        clockwise = 1;
    }
    while turn_times > 0 {
        let mut tmp = wp[0];
        wp[0] = wp[1] * clockwise;
        wp[1] = tmp * -1 * clockwise;

        turn_times -= 1;
    }
}