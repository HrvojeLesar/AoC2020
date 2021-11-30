use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut valid_passwords_part_1 = 0;
    let mut valid_passwords_part_2 = 0;
    
    for l in reader.lines() {
        let line = l.unwrap();
        let split_line: Vec<&str> = line.split(|c| c == '-' || c == ' ').collect();
        let min = split_line[0].parse::<u32>().unwrap();
        let max = split_line[1].parse::<u32>().unwrap();
        let pass_char = split_line[2].chars().next().unwrap();

        let mut counter = 0;
        let characters = split_line[3].chars().collect::<Vec<char>>();
        for chr in &characters {
            if *chr == pass_char {
                counter += 1;
            }
        }
        
        if counter >= min && counter <= max {
            valid_passwords_part_1 += 1;
        }

        let mut counter_part_2 = 0;
        if characters[min as usize - 1] == pass_char {
            counter_part_2 += 1
        }
        if characters[max as usize - 1] == pass_char {
            counter_part_2 += 1
        }
        if counter_part_2 == 1 {
            valid_passwords_part_2 += 1;
        }
    }

    println!("Valid passwords part 1:  {}", valid_passwords_part_1);
    println!("Valid passwords part 2:  {}", valid_passwords_part_2);
}
