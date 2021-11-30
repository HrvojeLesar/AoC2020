use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

static REQUIRED_PASSPORT_FIELDS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut passport_elements: HashMap<String, String> = HashMap::new();
    let mut valid_passports_part_1 = 0;
    let mut valid_passports_part_2 = 0;
    for l in reader.lines() {
        let line: String = l.unwrap();
        
        let pairs: Vec<&str> = line.split(|c| c == ':' || c == ' ').collect();
        for i in 0..pairs.len() - 1 as usize {
            passport_elements.insert(pairs[i].to_string(), pairs[i + 1].to_string());
        }

        // reset map
        if line == "" {
            let is_valid = is_passport_valid(&passport_elements);
            valid_passports_part_1 += is_valid;

            if is_valid == 1 {
                valid_passports_part_2 += are_fields_valid(&passport_elements);
            }
            
            passport_elements.clear();
        }
    }

    println!("Part 1: {}", valid_passports_part_1);
    println!("Part 2: {}", valid_passports_part_2);
}

fn is_passport_valid(map: &HashMap<String, String>) -> u32 {
    for field in &REQUIRED_PASSPORT_FIELDS {
        if map.get(&field.to_string()) == None {
            return 0;
        }
    }
    return 1;
}

fn are_fields_valid(map: &HashMap<String, String>) -> u32 {
    for field in &REQUIRED_PASSPORT_FIELDS {
        let value: &String = map.get(&field.to_string()).unwrap();
        match *field {
            "byr" => {
                if value.len() < 4 {
                    return 0;
                }
                let value = value.parse::<u32>().unwrap();
                if value < 1920 || value > 2002 {
                    return 0;
                }
            },
            "iyr" => {
                if value.len() < 4 {
                    return 0;
                }
                let value = value.parse::<u32>().unwrap();
                if value < 2010 || value > 2020 {
                    return 0;
                }
            },
            "eyr" => {
                if value.len() < 4 {
                    return 0;
                }
                let value = value.parse::<u32>().unwrap();
                if value < 2020 || value > 2030 {
                    return 0;
                }
            },
            "hgt" => {
                if value.contains("cm") {
                    let height_value: Vec<&str> = value.split("cm").collect();
                    let parsed_value = height_value[0].to_string().parse::<u32>().unwrap();
                    if parsed_value < 150 || parsed_value > 193 {
                        return 0;
                    }
                } else if value.contains("in"){
                    let height_value: Vec<&str> = value.split("in").collect();
                    let parsed_value = height_value[0].to_string().parse::<u32>().unwrap();
                    if parsed_value < 59 || parsed_value > 76 {
                        return 0;
                    }
                } else {
                    return 0;
                }
            },
            "hcl" => {
                if value.len() != 7 {
                    return 0;
                }
                let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                if !re.is_match(value) {
                    return 0;
                }
            },
            "ecl" => {
                match value.as_str() {
                    "amb"| "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                    _ => return 0,
                }
            },
            "pid" => {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                if !re.is_match(value) {
                    return 0;
                }
            },
            _ => { return 0; }
        }
    }
    return 1;
}

#[test]
fn valid_passport() {
    let mut map = HashMap::new();
    map.insert("byr".to_string(), "str".to_string());
    map.insert("iyr".to_string(), "str".to_string());
    map.insert("eyr".to_string(), "str".to_string());
    map.insert("hgt".to_string(), "str".to_string());
    map.insert("hcl".to_string(), "str".to_string());
    map.insert("ecl".to_string(), "str".to_string());
    map.insert("pid".to_string(), "str".to_string());
    assert_eq!(is_passport_valid(&map), 1);
}

#[test]
fn invalid_passport() {
    let mut map = HashMap::new();
    map.insert("byr".to_string(), "str".to_string());
    map.insert("iyr".to_string(), "str".to_string());
    map.insert("hgt".to_string(), "str".to_string());
    map.insert("hcl".to_string(), "str".to_string());
    map.insert("ecl".to_string(), "str".to_string());
    map.insert("pid".to_string(), "str".to_string());
    assert_eq!(is_passport_valid(&map), 0);
}

#[test]
fn regex_test() {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    assert_eq!(re.is_match("#cfa07d"), true);
    assert_eq!(re.is_match("#cfa07k49"), false);
}

#[test]
fn regex_test_2() {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    assert_eq!(re.is_match("123456789"), true);
    assert_eq!(re.is_match("19832894984"), false);
    assert_eq!(re.is_match("19832894a"), false);
}