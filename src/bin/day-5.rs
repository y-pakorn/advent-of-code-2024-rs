use std::{cmp::Ordering, env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-5-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-5-input-t.txt");

type Number = usize;

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let mut splitted = input.split("\n\n");
    let order = splitted
        .next()
        .expect("Failed to parse order")
        .lines()
        .map(|l| {
            let mut splitted = l.trim().split("|");
            let first = splitted
                .next()
                .expect("Failed to parse first")
                .parse::<Number>()
                .expect("Invalid first number");
            let second = splitted
                .next()
                .expect("Failed to parse second")
                .parse::<Number>()
                .expect("Invalid second number");
            (first, second)
        })
        .collect::<Vec<_>>();
    let updates = splitted
        .next()
        .expect("Failed to parse updates")
        .lines()
        .map(|l| {
            l.trim()
                .split(",")
                .map(|e| e.parse::<Number>())
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to parse update")
        })
        .collect::<Vec<_>>();

    let mut grid = [[false; 100]; 100];
    for (first, second) in &order {
        grid[*second][*first] = true;
    }

    let mut total = 0;
    let mut total_wrong = 0;
    for update in &updates {
        // check if update is sorted by order
        let mut cur_index = 0;
        let mut is_sorted = true;
        'o: while cur_index < update.len() {
            for i in cur_index..update.len() {
                if grid[update[cur_index]][update[i]] {
                    is_sorted = false;
                    break 'o;
                }
            }
            cur_index += 1;
        }

        match is_sorted {
            true => {
                total += update[update.len() / 2];
            }
            false => {
                // sort the wrong update by order
                let mut wrong = update.clone();
                wrong.sort_by(|a, b| match grid[*a][*b] {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                });
                total_wrong += wrong[wrong.len() / 2];
            }
        }
    }

    println!("Total: {}", total);
    println!("Total wrong: {}", total_wrong);
    println!("Elapsed: {:.4}s", now.elapsed().as_secs_f64());
}
