use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

static mut FOUND: bool = false;

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut map: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for l in reader.lines() {
        let line: String = l.unwrap();

        let mut line_iterator = line.split("bags contain");
        let bag_name: String = line_iterator.next().unwrap().trim().to_string();
        let other: Vec<&str> = line_iterator.next().unwrap().trim().split(" ").collect();
        let mut contains: HashMap<String, u32> = HashMap::new();
        for i in 0..(other.len() / 4) {
            let num = other[i * 4].parse::<u32>().unwrap();
            let name = other[i * 4 + 1].to_owned() + &" ".to_owned() + other[i * 4 + 2];
            contains.insert(name, num);
        }
        map.insert(bag_name, contains);
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for key in map.keys() {
        unsafe {
            read_bags(&map, key, &"shiny gold".to_owned());
            if FOUND {
                part_1 += 1;
                FOUND = false;
            }
        }
    }

    bags(&map, &"shiny gold".to_owned(), &mut part_2, 1);

    println!("{}", part_1);
    println!("{}", part_2);

}

fn read_bags(map: &HashMap<String, HashMap<String, u32>>, key: &String, search_for: &String) {
    unsafe {
        if FOUND {
            return;
        }
        let values = match map.get(key) {
            Some(v) => v,
            None => return,
        };
        
        for key in values.keys() {
            read_bags(map, key, search_for);
        }
        
        if values.get(search_for).is_some() {
            FOUND = true;
        }
    }
}

fn bags(map: &HashMap<String, HashMap<String, u32>>,  key: &String, p2: &mut u32, level: u32) {
    let values = match map.get(key) {
        Some(v) => v,
        None => return,
    };

    for (key, value) in values.iter() {
        bags(map, key, p2, *value * level);
    }
    if level > 1 {
        *p2 += level;
    }
}
    