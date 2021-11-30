use std::fs::File;
use std::io::{BufRead, BufReader};

use std::time::Instant;

fn main() {
	let start = Instant::now();
	
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut program = Vec::new();
    for l in reader.lines() {
        let line: String = l.unwrap();
        let mut line_iter = line.split(" ");
        program.push((line_iter.next().unwrap().to_owned(), line_iter.next().unwrap().parse::<i32>().unwrap(), 0));
    }

    println!("Part 1: {}", run_program(program.clone()).0);

    let mut program_clone = program.clone();

    for i in 0..program.len() - 1 {
        if program_clone[i].0 == "jmp".to_owned() {
            program_clone[i].0 = "nop".to_owned();
        } else if program_clone[i].0 == "nop".to_owned() {
            program_clone[i].0 = "jmp".to_owned();
        }
        let (acc, er) = run_program(program_clone.clone());
        if er == false {
            println!("Part 2: {}", acc);
            break;
        }
        program_clone = program.clone();
    }
	let duration = start.elapsed();
	println!("{:?}", duration);
}

fn run_program(mut program: Vec<(String, i32, u32)>) -> (i32, bool) {
    let mut acc = 0;
    let mut i: i32 = 0;
    let mut early_terminate = false;
    loop {
        if i as usize >= program.len() {
            break;
        }
        if program[i as usize].2 == 1 {
            early_terminate = true;
            break;
        }
        program[i as usize].2 = 1;
        match program[i as usize].0.as_str() {
            "acc" => { acc += program[i as usize].1; i += 1; },
            "jmp" => i += program[i as usize].1,
            "nop" | _ => i += 1
        }
    }
    return (acc, early_terminate);
}