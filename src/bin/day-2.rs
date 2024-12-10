use std::{cmp::Ordering, collections::HashSet, env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-2-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-2-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let levels = input
        .lines()
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.parse::<u32>().expect("Unable to parse"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_level_indexes = HashSet::new();
    'o: for (idx, level) in levels.iter().enumerate() {
        match level[0].cmp(&level[1]) {
            Ordering::Equal => continue 'o,
            ord => {
                for i in 0..level.len() - 1 {
                    match (level[i].cmp(&level[i + 1]), level[i].abs_diff(level[i + 1])) {
                        (o, d) if o == ord && d >= 1 && d <= 3 => {}
                        _ => continue 'o,
                    }
                }
                safe_level_indexes.insert(idx);
            }
        }
    }

    let mut safe_level_with_one_removal = HashSet::new();
    for (idx, level) in levels
        .iter()
        .enumerate()
        .filter(|(idx, _)| !safe_level_indexes.contains(idx))
    {
        'p: for ri in 0..level.len() {
            let mut removed = level.clone();
            removed.remove(ri);

            match removed[0].cmp(&removed[1]) {
                Ordering::Equal => continue 'p,
                ord => {
                    for i in 0..removed.len() - 1 {
                        match (
                            removed[i].cmp(&removed[i + 1]),
                            removed[i].abs_diff(removed[i + 1]),
                        ) {
                            (o, d) if o == ord && d >= 1 && d <= 3 => {}
                            _ => continue 'p,
                        }
                    }

                    safe_level_with_one_removal.insert(idx);
                    break 'p;
                }
            }
        }
    }

    println!("Safe count: {}", safe_level_indexes.len());
    println!(
        "Safe with one removal count: {}",
        safe_level_with_one_removal.len() + safe_level_indexes.len()
    );
    println!("Elapsed: {:.4}s", now.elapsed().as_secs_f64());
}
