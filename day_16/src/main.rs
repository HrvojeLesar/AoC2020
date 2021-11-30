use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::time::Instant;


fn main() {
    let start = Instant::now();

    let mut map: HashMap<u32, Vec<String>> = HashMap::new();
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut nerby_tickets: Vec<(Vec<u32>, bool)> = Vec::new();
    let mut classes: Vec<String> = Vec::new();

    let mut state = 0;
    for l in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line: String = l.unwrap();
        if line.len() == 0 {
            state += 1;
            continue;
        }

        match state {
            0 => {
                let mut split_line = line.split(": ");
                let var_name = split_line.next().unwrap().to_owned();
                let or_split = split_line.next().unwrap().split(" or ").collect::<Vec<&str>>();
                if !classes.contains(&var_name) {
                    classes.push(var_name.clone());
                }
                for j in 0..2 {
                    let from_to: Vec<u32> = or_split[j].split("-").map(|val| val.parse::<u32>().unwrap()).collect();
                    for i in from_to[0]..=from_to[1] {
                        match map.get_mut(&i) {
                            Some(s) => {
                                if !s.contains(&var_name) {
                                    s.push(var_name.clone());
                                }
                                let v = s.clone();
                                map.insert(i, v);
                            },
                            None => {
                                let mut v: Vec<String> = Vec::new();
                                v.push(var_name.clone());
                                map.insert(i, v);
                            }
                        }
                    }   
                }
            },
            1 => {
                if !line.starts_with("y") {
                    my_ticket = line.split(",").map(|num| num.parse::<u32>().unwrap()).collect();
                }
            },
            2 => if !line.starts_with("n") {
                nerby_tickets.push((line.split(",").map(|num| num.parse::<u32>().unwrap()).collect(), true));
            },
            _ => (),
        }
    }

    let mut part_1 = 0;
    let mut i = 0;
    for (ticket, _is_valid) in nerby_tickets.clone() {
        for numbers in ticket {
            if map.get(&numbers).is_none() {
                part_1 += numbers;
                nerby_tickets[i].1 = false;
            }
        }
        i += 1;
    }

    println!("Part 1: {:?}", part_1);

    let mut class_order: Vec<String> = vec!["".to_owned();classes.len()];

    let mut i = Vec::new();
    for j in 0..class_order.len() {
        i.push(j);
    }

    let mut ind = 0;
    let mut kek = 0;
    let last = i.len() - 1;

    while i.len() > 0 {
        let mut possible_classes: Vec<String> = Vec::new();
        for (ticket, is_valid) in &nerby_tickets {
            if *is_valid {
                if possible_classes.is_empty() {
                    possible_classes = map.get(&ticket[ind]).unwrap().clone();
                } else if possible_classes.len() > 1 {
                    let ticket_class = map.get(&ticket[ind]).unwrap().clone();
                    let mut pc: Vec<String> = Vec::new();
                    for j in 0..ticket_class.len() {
                        if possible_classes.contains(&ticket_class[j]) && !class_order.contains(&ticket_class[j]){
                            pc.push(ticket_class[j].clone());
                        }
                    }
                    possible_classes = pc;
                } else {
                    break;
                }
            }
        }
        if possible_classes.len() == 1 || last == ind {
            class_order[ind] = possible_classes.last().unwrap().clone();
            i.remove(get_index(ind, &i));
            if i.len() > 0 {
                kek = 0;
                ind = i[0];
            }
        } else {
            ind = i[(kek + 1) % i.len()];
            kek += 1;
        }
    }
    
    let mut part_2: u64 = 1;
    let mut i = 0;
    for cls in class_order {
        if cls.contains("departure") {
            part_2 *= my_ticket[i] as u64;
        }   
        i += 1;
    }

    println!("Part 2: {:?}", part_2);
    println!("Duration: {:?}", start.elapsed());
}


fn get_index(target: usize, v: &Vec::<usize>) -> usize {
    let mut i = 0;
    for t in v {
        if *t == target {
            break;
        }
        i += 1;
    }
    i
}