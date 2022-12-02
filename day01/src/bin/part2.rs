use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => return println!("{}", err),
    };
    let reader = BufReader::new(file);

    let mut cur_sum: i32 = 0;
    let mut top_cal_stack: Vec<i32> = Vec::new();

    for line in reader.lines() {

      match line.unwrap().parse::<i32>() {
        Ok(value) => cur_sum += value,
        Err(_) => {
            top_cal_stack.push(cur_sum);
            cur_sum = 0;
        }
      }
    }

    let n = 3;
    let mut total_top_n: i32 = 0;
    top_cal_stack.sort();
    for _i in 0..n {
      total_top_n += top_cal_stack.pop().unwrap_or(0);
    }

    println!("{:?}", top_cal_stack);
    println!("Top 3 Elfs are carrying {} total calories", total_top_n);

}