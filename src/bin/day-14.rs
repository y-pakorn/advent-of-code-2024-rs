use std::{env::args, time::Instant};

use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day-14-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-14-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").expect("Invalid regex");
    let mut robots = re
        .captures_iter(input)
        .map(|robot| {
            let (_, [px, py, vx, vy]) = robot.extract();
            (
                (
                    px.parse::<usize>().expect("Invalid number"),
                    py.parse::<usize>().expect("Invalid number"),
                ),
                (
                    vx.parse::<isize>().expect("Invalid number"),
                    vy.parse::<isize>().expect("Invalid number"),
                ),
            )
        })
        .collect::<Vec<_>>();
    let mut second_robots = robots.clone();

    let x_size = robots.iter().map(|((x, _), _)| x).max().unwrap() + 1;
    let y_size = robots.iter().map(|((_, y), _)| y).max().unwrap() + 1;

    let target_seconds = 100;

    let construct_grid = |robots: &[((usize, usize), (isize, isize))]| {
        let mut grid = vec![vec![0; x_size]; y_size];
        for ((x, y), _) in robots {
            grid[*y][*x] += 1;
        }
        grid
    };

    let print_grid = |robots: &[((usize, usize), (isize, isize))]| {
        let grid = construct_grid(robots);
        for y in 0..y_size {
            for x in 0..x_size {
                match grid[y][x] {
                    0 => print!("."),
                    i => print!("{i}"),
                }
            }
            println!();
        }
    };

    let step = |robots: &mut [((usize, usize), (isize, isize))]| {
        for ((x, y), (vx, vy)) in robots.iter_mut() {
            let new_x = *x as isize + *vx;
            let new_y = *y as isize + *vy;

            // wrap around
            *x = if new_x < 0 {
                x_size as isize + new_x
            } else {
                new_x % x_size as isize
            } as usize;
            *y = if new_y < 0 {
                y_size as isize + new_y
            } else {
                new_y % y_size as isize
            } as usize;
        }
    };

    for _ in 0..target_seconds {
        step(&mut robots);
    }

    let grid = construct_grid(&robots);
    let safety_factor = vec![
        (0..x_size / 2, 0..y_size / 2),
        (x_size.div_ceil(2)..x_size, 0..y_size / 2),
        (0..x_size / 2, y_size.div_ceil(2)..y_size),
        (x_size.div_ceil(2)..x_size, y_size.div_ceil(2)..y_size),
    ]
    .iter()
    .map(|(x_range, y_range)| {
        grid.iter()
            .skip(y_range.start)
            .take(y_range.end - y_range.start)
            .map(|row| {
                row.iter()
                    .skip(x_range.start)
                    .take(x_range.end - x_range.start)
                    .sum::<usize>()
            })
            .sum::<usize>()
    })
    .product::<usize>();

    let mut second_seconds = 0;
    for _ in 0..100 {
        step(&mut second_robots);
    }
    for seconds in 100..y_size * x_size {
        step(&mut second_robots);
        let grid = construct_grid(&second_robots);
        // look for atleast 2 rows that have 16 or more robots
        if grid
            .iter()
            .filter(|row| row.chunks(16).any(|c| c.iter().all(|x| *x > 0)))
            .take(2)
            .count()
            >= 2
        {
            print_grid(&second_robots);
            second_seconds = seconds + 1;
            break;
        }
    }

    println!("Safety factor: {}", safety_factor);
    println!("Seconds: {}", second_seconds);
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
