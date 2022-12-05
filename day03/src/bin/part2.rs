use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => return println!("{}", err),
    };
    let reader = BufReader::new(file);
    let mut badges: Vec<u8> = Vec::new();
    let mut iter: u16 = 0;
    let mut items: HashMap<u8, u8> = HashMap::new();
    for line in reader.lines() {
        match line.unwrap().parse::<String>() {
            Ok(rucksack) => {
                for item in rucksack.as_bytes() {
                    if iter % 3 == 0 {
                        if items.contains_key(item) {
                            continue;
                        } else {
                            items.insert(item.to_owned(), 1);
                        }
                    } else if iter % 3 == 1 {
                        if items.contains_key(item) {
                            let cur_count = items[item];
                            if cur_count == 1 {
                                items.insert(*item, cur_count + 1);
                            }
                        }
                    } else {
                        if items.contains_key(item) {
                            if items[item] == 2 {
                                badges.push(item.to_owned());
                                items.clear();
                                break;
                            }
                        }
                    }
                }
                iter += 1;
            }

            Err(err) => println!("Error reading line {}", err),
        }
    }
    let mut priority_score: u32 = 0;
    for item in badges {
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
