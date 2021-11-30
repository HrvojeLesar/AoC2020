use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut part_1 = 0u64;
    let mut part_2 = 0u64;
    for l in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line: String = l.unwrap();
        let line = line.split_whitespace().collect::<String>();
        part_1 += parse_expression(&line);
        part_2 += parse_expression_p2(&line);
    }

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);

    println!("Duration: {:?}", start.elapsed());
}

fn parse_expression(line: &String) -> u64 {

    let mut states: Vec<(u64, char)> = Vec::new();
    let mut current_total = 0u64;
    let mut current_op = '+';

    for chr in line.chars() {
        match chr {
            '(' => {
                states.push((current_total.clone(), current_op.clone()));
                current_total = 0;
                current_op = '+';
            },
            ')' => {
                let state = states.pop().unwrap();
                if state.1 == '+' {
                    current_total += state.0;
                } else if state.1 == '*' {
                    current_total *= state.0;
                }
            },
            '+' | '*' => {
                current_op = chr.clone();
            },
            num => {
                let num = num.to_digit(10).unwrap() as u64;
                if current_op == '+' {
                    current_total += num;
                } else if current_op == '*' {
                    current_total *= num;
                }
            },
        }
    }
    return current_total;
}

fn parse_expression_p2(line: &String) -> u64 {

    let mut states: Vec<(u64, char, bool)> = Vec::new();
    let mut current_total = 0u64;
    let mut current_op = '+';

    for chr in line.chars() {
        match chr {
            '(' => {
                states.push((current_total.clone(), current_op.clone(), true));
                current_total = 0;
                current_op = '+';
            },
            ')' => {
                while {
                    let state = states.pop().unwrap();
                    if state.1 == '+' {
                        current_total += state.0;
                    } else if state.1 == '*' {
                        current_total *= state.0;
                    }
                    !state.2
                } {

                }
            },
            '*' | '+' => {
                if chr == '*' && current_total != 0 {
                    states.push((current_total.clone(), chr.clone(), false));
                    current_total = 0;
                }
                current_op = '+';
            },
            num => {
                let num = num.to_digit(10).unwrap() as u64;
                if current_op == '+' {
                    current_total += num;
                } else if current_op == '*' {
                    current_total *= num;
                }
            },
        }
    }

    while !states.is_empty() {
        let state = states.pop().unwrap();
        if state.1 == '+' {
            current_total += state.0;
        } else if state.1 == '*' {
            current_total *= state.0;
        }
    }

    return current_total;
}