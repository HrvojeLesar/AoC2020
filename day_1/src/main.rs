use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut less_than_1000: Vec<u32> = Vec::new();
    let numbers: Vec<u32> = file.split('\n')
                                .map(|num| {
                                    let parsed_number = num.parse::<u32>().unwrap();
                                    if parsed_number <= 1000 {
                                        less_than_1000.push(parsed_number);
                                    }
                                    parsed_number
                                })
                                .collect();

    for num in &numbers {
        for add in &less_than_1000 {
            if num + add == 2020 {
                println!("Part 1:");
                println!("{} * {} = {}", num, add, num * add);
            }
        }
    }

    for num in &numbers {
        for add in &less_than_1000 {
            for add2 in &less_than_1000 {
                if num + add + add2 == 2020 {
                    println!("Part 2:");
                    println!("{} * {} * {} = {}", num, add, add2, num * add * add2);
                }
            }
        }
    }
}
