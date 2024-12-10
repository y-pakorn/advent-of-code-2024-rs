use std::{collections::HashMap, env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-1-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-1-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|l| {
            let mut w = l
                .split_whitespace()
                .map(|w| w.parse::<u32>().expect("Failed to parse input"));
            let left = w.next().expect("Failed to parse left");
            let right = w.next().expect("Failed to parse right");
            (left, right)
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>();

    let right_occurances: HashMap<u32, u32> = right.iter().fold(HashMap::new(), |mut acc, &r| {
        *acc.entry(r).or_insert(0) += 1;
        acc
    });
    let mut sim_score = 0;
    left.iter().for_each(|l| {
        if let Some(&r) = right_occurances.get(l) {
            sim_score += l * r;
        }
    });

    println!("Sum: {}", sum);
    println!("Sim score: {}", sim_score);
    println!("Elapsed: {:.4}s", now.elapsed().as_secs_f64());
}
