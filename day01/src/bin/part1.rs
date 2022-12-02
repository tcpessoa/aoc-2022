use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => return println!("{}", err),
    };
    let reader = BufReader::new(file);

    let mut cur_sum: i32 = 0;
    let mut max_sum: i32 = 0;
    let mut index_elf_max_cal: i32 =0;
    let mut index_cur_elf: i32 =0;

    for line in reader.lines() {

      match line.unwrap().parse::<i32>() {
        Ok(value) => cur_sum += value,
        Err(_) => {
            if cur_sum > max_sum {
                max_sum = cur_sum;
                index_elf_max_cal = index_cur_elf;
            } 
            cur_sum = 0;
            index_cur_elf += 1;

        }
      }
    }

    println!("Elf number: {}", index_elf_max_cal);
    println!("is carrying {} total calories", max_sum);

}