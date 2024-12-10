use std::{collections::HashSet, env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-10-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-10-input-t.txt");

fn pathfind(grid: &[Vec<u32>], cur_loc: (usize, usize)) -> (HashSet<(usize, usize)>, usize) {
    match grid[cur_loc.1][cur_loc.0] {
        9 => (HashSet::from([cur_loc]), 1),
        cur_val => {
            // search up, down, left, right
            let mut scores = HashSet::new();
            let mut rating = 0;

            for (dx, dy) in [
                (0, -1), // up
                (0, 1),  // down
                (-1, 0), // left
                (1, 0),  // right
            ] {
                let (x, y) = (cur_loc.0 as i32 + dx, cur_loc.1 as i32 + dy);
                if x >= 0
                    && x < grid[0].len() as i32
                    && y >= 0
                    && y < grid.len() as i32
                    && grid[y as usize][x as usize] == cur_val + 1
                {
                    let (score, r) = pathfind(grid, (x as usize, y as usize));
                    scores.extend(score);
                    rating += r;
                }
            }

            (scores, rating)
        }
    }
}

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Could not parse number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut scores = 0;
    let mut rating = 0;
    for zero_loc in grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &v)| v == 0)
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .flatten()
    {
        let (cur_score, cur_rating) = pathfind(&grid, zero_loc);
        scores += cur_score.len();
        rating += cur_rating;
    }

    println!("Scores: {}", scores);
    println!("Rating: {}", rating);
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
