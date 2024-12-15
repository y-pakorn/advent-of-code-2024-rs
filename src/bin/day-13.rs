use std::{env::args, time::Instant};

use nalgebra::{Matrix2, Vector2};
use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day-13-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-13-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )
    .expect("Invalid regex");
    let machines = re
        .captures_iter(input)
        .map(|found| {
            let (_, [adx, ady, bdx, bdy, x, y]) = found.extract();
            (
                (
                    x.parse::<u64>().expect("Invalid number"),
                    y.parse::<u64>().expect("Invalid number"),
                ),
                [
                    (
                        adx.parse::<u64>().expect("Invalid number"),
                        ady.parse::<u64>().expect("Invalid number"),
                    ),
                    (
                        bdx.parse::<u64>().expect("Invalid number"),
                        bdy.parse::<u64>().expect("Invalid number"),
                    ),
                ],
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut sum2 = 0;
    for machine in machines {
        let ((x, y), [(adx, ady), (bdx, bdy)]) = machine;

        // find number of a and b button presses to reach prize (x, y)
        // each a and b button press moves the machine by (adx, ady) and (bdx, bdy) respectively
        // the machine starts at (0, 0)
        let max_steps = 100;
        let mut used = vec![];
        for a in 0..max_steps {
            for b in 0..max_steps {
                if a * adx + b * bdx == x && a * ady + b * bdy == y {
                    used.push(3 * a + b);
                }
            }
        }
        used.sort_unstable();
        sum += used.get(0).unwrap_or(&0);

        // for large x,y values, use matrix approach
        // | adx bdx | | na | = | x |
        // | ady bdy | | nb | = | y |
        // so
        // | na | = | adx bdx |^-1 | x |
        // | nb | = | ady bdy |    | y |
        let (corrected_x, corrected_y) = (x + 10000000000000, y + 10000000000000);
        if let Some(dyx_inv) =
            Matrix2::new(adx as f64, bdx as f64, ady as f64, bdy as f64).try_inverse()
        {
            let xy = Vector2::new(corrected_x as f64, corrected_y as f64);
            let nab = dyx_inv * xy;
            if nab
                .iter()
                .all(|&n| n.fract() < 0.0001 || n.fract() > 0.9999)
            {
                sum2 += nab[0].round() as u64 * 3 + nab[1].round() as u64;
            }
        }
    }

    println!("Sum Tokens: {}", sum);
    println!("Sum Tokens (Part Two): {}", sum2);
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
