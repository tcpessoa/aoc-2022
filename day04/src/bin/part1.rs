use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => return println!("{}", err),
    };
    let reader = BufReader::new(file);
    let mut counter: u32 = 0;
    for line in reader.lines() {
        match line.unwrap().parse::<String>() {
            Ok(pair) => {
                let sections: Vec<&str> = pair.split(",").collect();
                let mut limits: Vec<Vec<u32>> = Vec::new();
                sections.into_iter().for_each(|section| {
                    limits.push(section.split("-").map(|s: &str| s.parse().unwrap()).collect());
                });
                println!("{:?}", limits);
                if limits[0][0] <= limits[1][0] && limits[0][1] >= limits[1][1] {
                    println!("Contains");
                    counter += 1;
                } 
                else if limits[1][0] <= limits[0][0] && limits[1][1] >= limits[0][1] {
                    println!("Contains");
                    counter += 1;
                }
                else if limits[0][0] <= limits[1][0] && limits[0][1] >= limits[1][0] {
                    println!("Contains");
                    counter += 1;
                }
                else if limits[1][0] <= limits[0][0] && limits[1][1] >= limits[0][0] {
                    println!("Contains");
                    counter += 1;
                }

            }
            Err(err) => println!("Error reading line {}", err),
        }
    }

    println!("{:?}", counter);
    // guessed 590 wrong, too high
    // didn't parse &str to u32, that was causing issues
}
