use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => return println!("{}", err),
    };
    let reader = BufReader::new(file);

    let shape_score: HashMap<&str, u32> = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let opponent_shape_wins_to = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);
    let opponent_shape_loses_to = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);

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
                let mut my_shape_score: u32 = 0;

                let opponent_shape = round[0];
                let desired_outcome = round[1];
                if desired_outcome == "Y" {
                    my_shape_score = shape_score[&opponent_shape].into();
                    round_score = 3;
                } else if desired_outcome == "Z" {
                    let my_shape = opponent_shape_loses_to[&opponent_shape];
                    my_shape_score = shape_score[&my_shape].into();
                    round_score = 6;
                } else {
                    let my_shape = opponent_shape_wins_to[&opponent_shape];
                    my_shape_score = shape_score[&my_shape].into();
                }
                current_score += my_shape_score;
                current_score += round_score;
                total_score += current_score;
            }
            Err(err) => println!("Error reading line {}", err),
        }
    }
    println!("Total score: {}", total_score)
}
