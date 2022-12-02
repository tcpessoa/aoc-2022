use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => return println!("{}", err),
    };
    let reader = BufReader::new(file);
    let shape_score: HashMap<&str, u32> = HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let shape_wins_to = HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")]);
    let equivalent_shape = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);

    let mut total_score: u32 = 0;
    for line in reader.lines() {
        match line.unwrap().parse::<String>() {
            Ok(value) => {
                let split = value.split_whitespace();
                let mut round: Vec<&str> = Vec::new();
                for shape in split {
                    round.push(shape);
                }
                let mut current_score: u32 = 0;
                let mut round_score: u32 = 0;
                let opponent_shape = round[0];
                let my_shape = round[1];
                let my_shape_score: u32 = shape_score[&my_shape].into();
                current_score += my_shape_score;
                if opponent_shape == equivalent_shape[&my_shape] {
                    round_score = 3;
                } else if shape_wins_to[&my_shape] == opponent_shape {
                    round_score = 6;
                }
                current_score += round_score;
                total_score += current_score;
            }
            Err(err) => println!("Error reading line {}", err),
        }
    }
    println!("Total score: {}", total_score)
}

// 16442 is wrong