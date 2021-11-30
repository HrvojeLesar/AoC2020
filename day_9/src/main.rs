use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {

    let start = Instant::now();

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut sequence: Vec<u64> = Vec::new();
    for l in reader.lines() {
        let line: String = l.unwrap();
        sequence.push(line.parse().unwrap());
    }
    let invalid_number = attack_xmas(&sequence, 25);
    println!("Part 1: {}", invalid_number);
    let mut sum = sum_to_invalid(&sequence, invalid_number);
    sum.sort();
    println!("Part 2: {:?}", sum[0] + sum[sum.len() - 1]);

    println!("Duration: {:?}", start.elapsed());
}

fn attack_xmas(sequence: &Vec<u64>, mut preamble: u64) -> u64 {
    let mut pos = 0;
    while (pos as usize) < sequence.len() {
        let mut has_sum = false;
        for i in pos..preamble {
            if has_sum {
                break;
            }
            for j in (i + 1)..preamble {
                if sequence[i as usize] + sequence[j as usize] == sequence[preamble as usize] {
                    has_sum = true;
                    break;
                }
            }
        }
        if !has_sum {
            return sequence[preamble as usize];
        }
        pos += 1;
        preamble += 1;
    }
    return 0;
}

fn sum_to_invalid(sequence: &Vec<u64>, invalid_number: u64) -> Vec<u64> {
    for i in 0..sequence.len() {
        let mut sum = 0;
        let mut numbers = Vec::new();
        sum += sequence[i];
        numbers.push(sequence[i]);
        for j in (i + 1)..sequence.len() {
            sum += sequence[j];
            numbers.push(sequence[j]);
            if sum == invalid_number && numbers.len() > 1 {
                return numbers;
            }
        }
        numbers.clear();
    }

    return vec![0];
}