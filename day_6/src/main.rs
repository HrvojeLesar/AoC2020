use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
	let start = Instant::now();
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut map: HashMap<char, u32> = HashMap::new();
    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut members = 0;
    for l in reader.lines() {
        let line: String = l.unwrap();
        
        if line == "" {
            // check for each value eq to member num
            for (key, value) in map.iter() {
                if *value == members {
                    part_2 += 1;
                }
            }
            members = 0;
            map.clear();
            continue;
        }

        members += 1;
        line.chars().for_each(|chr| {
            match map.insert(chr, 1) {
                None => part_1 += 1,
                Some(n) => { map.insert(chr, n + 1); () },
            }
        });
    }

    for (_, value) in map.iter() {
        if *value == members {
            part_2 += 1;
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
	
	println!("{:?}", start.elapsed());
}