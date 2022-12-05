use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => return println!("{}", err),
    };
    let reader = BufReader::new(file);
    let mut repeated_items: Vec<u8> = Vec::new();
    for line in reader.lines() {
        match line.unwrap().parse::<String>() {
            Ok(rucksack) => {
                let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
                let mut items_comp1: HashMap<u8, u8> = HashMap::new();
                for item in comp1.as_bytes() {
                    items_comp1.insert(item.to_owned(), 1);
                }
                for item in comp2.as_bytes() {
                    if items_comp1.contains_key(&item) {
                        repeated_items.push(item.to_owned());
                        // need to break as there may be more than 1 occurence of repeated items
                        break;
                    }
                }
            }
            Err(err) => println!("Error reading line {}", err),
        }
    }
    let mut priority_score: u32 = 0;
    for item in repeated_items {
        let prev_score = priority_score;
        let letter = item as char;
        if letter.is_ascii_lowercase() {
            priority_score += item as u32 - 96; // a is 97, 97-96=1, offset the score
        } else {
            priority_score += item as u32 - 38; // A is 65, 65-38=27
        }
        let score = priority_score - prev_score;
        println!("{}, {}, {}", item, score, priority_score)
    }

    println!("{:?}", priority_score);
}
