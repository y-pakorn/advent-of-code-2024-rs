use std::{collections::HashMap, env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-11-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-11-input-t.txt");

type Number = u128;

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let stones = input
        .split_whitespace()
        .map(|x| x.parse::<Number>().expect("Invalid number"))
        .collect::<Vec<_>>();

    let blink_target = 75;

    let mut memo: HashMap<(Number, usize), usize> = HashMap::new();

    fn compute(
        stone: Number,
        remaining_blinks: usize,
        memo: &mut HashMap<(Number, usize), usize>,
    ) -> usize {
        if remaining_blinks == 0 {
            return 1;
        }
        if let Some(&result) = memo.get(&(stone, remaining_blinks)) {
            return result;
        }

        let result = if stone == 0 {
            compute(1, remaining_blinks - 1, memo)
        } else if stone.to_string().len() % 2 == 0 {
            let digits = stone.to_string();
            let mid = digits.len() / 2;
            let left = digits[..mid].parse::<Number>().unwrap();
            let right = digits[mid..].parse::<Number>().unwrap();
            compute(left, remaining_blinks - 1, memo) + compute(right, remaining_blinks - 1, memo)
        } else {
            compute(stone * 2024, remaining_blinks - 1, memo)
        };

        memo.insert((stone, remaining_blinks), result);
        result
    }

    let sum = stones
        .iter()
        .map(|&stone| compute(stone, blink_target, &mut memo))
        .sum::<usize>();

    println!("Sum: {}", sum);
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
