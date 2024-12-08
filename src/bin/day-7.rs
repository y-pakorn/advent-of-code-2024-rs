use std::{env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-7-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-7-input-t.txt");

type Number = u64;

fn evaluate_plus_mult(total: Number, var: &[Number], cur_total: Number, index: usize) -> bool {
    match index {
        // If is used, start with cur = first variable and index = 1
        0 => evaluate_plus_mult(total, var, var[0], 1),
        // If all variables are used, check if total is reached
        _ if index == var.len() => cur_total == total,
        // If not all variables are used, try adding first then multiplying
        _ => {
            evaluate_plus_mult(total, var, cur_total + var[index], index + 1)
                || evaluate_plus_mult(total, var, cur_total * var[index], index + 1)
        }
    }
}

fn evaluate_plus_mult_concat(
    total: Number,
    var: &[Number],
    cur_total: Number,
    index: usize,
) -> bool {
    match index {
        // If is used, start with cur = first variable and index = 1
        0 => evaluate_plus_mult_concat(total, var, var[0], 1),
        // If all variables are used, check if total is reached
        _ if index == var.len() => cur_total == total,
        // If not all variables are used, try adding first then multiplying, then concatenating
        _ => {
            // plus
            evaluate_plus_mult_concat(total, var, cur_total + var[index], index + 1)
                // multiply
                || evaluate_plus_mult_concat(total, var, cur_total * var[index], index + 1)
                // concat
                || evaluate_plus_mult_concat(total, var, format!("{}{}", cur_total, var[index]).parse::<Number>().expect("Invalid concatenated number"), index + 1)
        }
    }
}

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let equations = input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let total = parts
                .next()
                .expect("No total found")
                .parse::<Number>()
                .expect("Total is not a number");
            let var = parts
                .next()
                .map(|part| {
                    part.trim()
                        .split(" ")
                        .map(|part| part.parse::<Number>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .expect("No variable found")
                .expect("Variable is not a number");
            (total, var)
        })
        .collect::<Vec<_>>();

    let mut cur_total = 0;
    let mut cur_total_concat = 0;
    for (total, var) in equations {
        // Possible operation +, *. Find if equation is possible by atleast 1 permutation of
        // operations and using all variables, equation is always calculated left to right
        if evaluate_plus_mult(total, &var, 0, 0) {
            cur_total += total;
            cur_total_concat += total;
            continue;
        }

        // Add concat operation ||
        if evaluate_plus_mult_concat(total, &var, 0, 0) {
            cur_total_concat += total;
        }
    }
    println!("Total: {}", cur_total);
    println!("Total Concat: {}", cur_total_concat);
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
