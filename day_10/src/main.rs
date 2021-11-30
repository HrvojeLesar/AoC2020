use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut adapters: Vec<u32> = Vec::new();

    for l in reader.lines() {
        let line = l.unwrap();
        adapters.push(line.parse().unwrap());
    }
    adapters.sort();

    
    let mut diff_1 = 1;
    let mut diff_3 = 1;

    for i in 0..adapters.len() - 1{
        match adapters[i + 1] - adapters[i] {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => (),
        }
    }
    
    println!("Part 1: {}", diff_1 * diff_3);

    adapters.insert(0, 0);
    println!("Part 2: {}", p2(&mut adapters));
    
    println!("Running time: {:?}", start.elapsed());
}

fn p2(data: &mut Vec<u32>) -> u64 {
    data.reverse();

    let target = data[0] + 3;
    let mut occur_num = vec![0 as u64; target as usize + 1];
    occur_num[target as usize] = 1;

    for n in data {
        for i in *n + 1..=*n + 3 {
            occur_num[*n as usize] += occur_num[i as usize];
        }
    }
    occur_num[0]
}