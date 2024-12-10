use regex::Regex;
use std::{env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-3-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-3-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let mut sum = 0;
    let mut sum_enabled = 0;
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Invalid regex");
    for found in re.captures_iter(input) {
        match &found[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let lhs = found[1].parse::<u32>().expect("Invalid number");
                let rhs = found[2].parse::<u32>().expect("Invalid number");
                if enabled {
                    sum_enabled += lhs * rhs;
                }
                sum += lhs * rhs;
            }
        }
    }

    println!("Sum: {}", sum);
    println!("Sum enabled: {}", sum_enabled);
    println!("Elapsed: {:.4}s", now.elapsed().as_secs_f64());
}
