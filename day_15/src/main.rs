use std::fs::read_to_string;
use std::collections::HashMap;
use std::time::Instant;

fn main() {

    let start = Instant::now();

    // broj, first time 
    let sequence: Vec<(u32, bool)> = read_to_string("input.txt").unwrap().split(",").map(|n| (n.parse::<u32>().unwrap(), true)).collect();
    let mut turn = 1;

    // first time, last occur
    let mut map: HashMap<u32, (bool, [u32; 2])> = HashMap::new();
    for (val, _) in &sequence {
        map.insert(*val, (true, [turn, 0]));
        turn += 1;
    }

    let mut last_num = sequence.last().unwrap().0;

    while turn <= 2020 {
        if map.get(&last_num).unwrap().0 {
            let helper = match map.get(&0) {
                Some(s) => s.1,
                None => [0; 2]
            };
            map.insert(0, (!map.get(&0).is_some(), [turn, helper[0]]));
            last_num = 0;
        } else {

            let last_occur = map.get(&last_num).unwrap().1;

            let num = (turn - 1 - last_occur[1]) as u32;

            let mut last_occur_num = [0; 2];
            if let Some(n) = map.get(&num) {
                last_occur_num = n.1;
            }

            map.insert(num, (!map.get(&num).is_some(), [turn, last_occur_num[0]]));
            last_num = num;
        }

        turn += 1;
    }

    println!("Part 1: {:?}", last_num);
    
    while turn <= 30000000 {
        if map.get(&last_num).unwrap().0 {
            let helper = match map.get(&0) {
                Some(s) => s.1,
                None => [0; 2]
            };
            map.insert(0, (!map.get(&0).is_some(), [turn, helper[0]]));
            last_num = 0;
        } else {

            let last_occur = map.get(&last_num).unwrap().1;

            let num = (turn - 1 - last_occur[1]) as u32;

            let mut last_occur_num = [0; 2];
            if let Some(n) = map.get(&num) {
                last_occur_num = n.1;
            }

            map.insert(num, (!map.get(&num).is_some(), [turn, last_occur_num[0]]));
            last_num = num;
        }

        turn += 1;
    }
    
    println!("Part 2: {:?}", last_num);
    println!("Duration: {:?}", start.elapsed());
}