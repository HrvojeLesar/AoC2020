use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

fn main() {

    let start = Instant::now();
    let mut mask: Vec<(char, usize)> = Vec::new();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut memory_p2: HashMap<u64, u64> = HashMap::new();

    for l in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line: String = l.unwrap();
        if line.starts_with("mask") {
            mask.clear();
            let mut i = 0;
            for chr in line.split_at(7).1.chars() {
                mask.push((chr, i));
                i += 1;
            }
        } else {
            let re = Regex::new(r"[0-9]+").unwrap();
            let mut cap_iter = re.captures_iter(&line);
            let mem_address = cap_iter.next().unwrap().get(0).unwrap().as_str().parse::<u64>().unwrap();
            let value = cap_iter.next().unwrap().get(0).unwrap().as_str().parse::<u64>().unwrap();

            memory.insert(mem_address, apply_mask(&mask, value));
            apply_mask_p2(&mask, value, &mut memory_p2, &mem_address);
        }
    }
    
    println!("Part 1: {:?}", memory.values().sum::<u64>());

    let mut part2_sum = 0;
    memory_p2.values().for_each(|v| part2_sum += v);
    println!("Part 2: {:?}", part2_sum);


    println!("Duration: {:?}", start.elapsed());
}

fn apply_mask(mask: &Vec<(char, usize)>, value: u64) -> u64 {
    let mut real_value: Vec<char> = "000000000000000000000000000000000000".chars().collect();
    let mut val: Vec<char> = format!("{:b}", value).chars().collect();
    val.reverse();

    for i in 0..val.len() {
        real_value[i] = val[i];
    }

    real_value.reverse();
    for (val, pos) in mask {
        if *val == 'X' {
            continue;
        }
        real_value[*pos] = *val;
    }

    u64::from_str_radix(&real_value.iter().collect::<String>(), 2).unwrap()
}

fn apply_mask_p2(mask: &Vec<(char, usize)>, value: u64, memory: &mut HashMap<u64, u64>, address: &u64) {
    let mut real_value: Vec<char> = "000000000000000000000000000000000000".chars().collect();
    let mut addr: Vec<char> = format!("{:b}", address).chars().collect();
    addr.reverse();

    for i in 0..addr.len() {
        real_value[i] = addr[i];
    }
    
    real_value.reverse();
    
    for (val, pos) in mask {
        if *val == 'X' || *val == '0' {
            continue;
        }
        real_value[*pos] = *val;
    }

    let x_mask: Vec<&(char, usize)> = mask.iter().filter(|x| x.0.eq(&'X')).collect();

    let mut values_for_memory: Vec<u64> = Vec::new();
    for i in 0..2usize.pow(x_mask.len() as u32) {
        let mut perm = vec![0; x_mask.len()];
        let mut num: Vec<char> = format!("{:b}", i).chars().collect();
        num.reverse();

        for j in 0..num.len() {
            perm[j] = num[j].to_digit(10).unwrap();
        }
        for j in 0..x_mask.len() {
            real_value[x_mask[j].1] = perm[j].to_string().chars().next().unwrap();
        }
        values_for_memory.push(u64::from_str_radix(&real_value.iter().collect::<String>(), 2).unwrap())
    }
    for mem_val in values_for_memory {
        memory.insert(mem_val, value);
    }
}