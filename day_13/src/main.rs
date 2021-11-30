use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut l = BufReader::new(File::open("input.txt").unwrap()).lines();
    let earliest_timestamp = l.next().unwrap().unwrap().parse::<u64>().unwrap();
    let mut busses: Vec<u64> = Vec::new();
    let mut busses_p2: Vec<(u64, u64)> = Vec::new();
    let mut i: f32 = 0.0;
    l.next().unwrap().unwrap().split(|pat| pat == ',' || pat == 'x').for_each(|num| {
        if let Ok(n) = num.parse::<u64>() {
            busses.push(n);
            busses_p2.push((n, i as u64));
            i += 1.0;
        } else {
            i += 0.5;
        }
    });

    let (mut min, mut bus_id) = earliest_devisor(&busses[0], &earliest_timestamp);
    for i in 1..busses.len() {
        let (div, id) = earliest_devisor(&busses[i], &earliest_timestamp);
        if div < min {
            min = div;
            bus_id = id;
        }
    }

    println!("Part 1: {:?}", (min - earliest_timestamp) * bus_id);

    let mut timestamp = 0;
    let mut jmp = busses_p2[0].0;
    for i in 1..busses_p2.len() {
        while (timestamp + busses_p2[i].1) % busses_p2[i].0 != 0 {
            timestamp += jmp;
        }
        jmp *= busses_p2[i].0;
    }

    println!("Part 2: {}", timestamp);
    println!("Duration: {:?}", start.elapsed());
}

fn earliest_devisor(num: &u64, target: &u64) -> (u64, u64) {
    let mut divisor = num.clone();
    while divisor < *target {
        divisor += num;
    }
    return (divisor, num.clone())
}