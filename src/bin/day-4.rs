use std::{env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-4-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-4-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    // word search input for "XMAS" horizontally, vertically and diagonally. "XMAS" can be
    // overlapping and reversed. -> count all instances of "XMAS" in the input
    // So: horizontal, horizontal reverse, vertical, vertical reverse, diagonal, diagonal reverse
    // 'X' = 0, 'M' = 1, 'A' = 2, 'S' = 3
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // horizontal
            if j + 3 < grid[i].len() {
                if (grid[i][j] == 'X'
                    && grid[i][j + 1] == 'M'
                    && grid[i][j + 2] == 'A'
                    && grid[i][j + 3] == 'S')
                    || (grid[i][j] == 'S'
                        && grid[i][j + 1] == 'A'
                        && grid[i][j + 2] == 'M'
                        && grid[i][j + 3] == 'X')
                {
                    count += 1;
                }
            }
            // vertical
            if i + 3 < grid.len() {
                if (grid[i][j] == 'X'
                    && grid[i + 1][j] == 'M'
                    && grid[i + 2][j] == 'A'
                    && grid[i + 3][j] == 'S')
                    || (grid[i][j] == 'S'
                        && grid[i + 1][j] == 'A'
                        && grid[i + 2][j] == 'M'
                        && grid[i + 3][j] == 'X')
                {
                    count += 1;
                }
            }
            // diagonal
            if i + 3 < grid.len() && j + 3 < grid[i].len() {
                if (grid[i][j] == 'X'
                    && grid[i + 1][j + 1] == 'M'
                    && grid[i + 2][j + 2] == 'A'
                    && grid[i + 3][j + 3] == 'S')
                    || (grid[i][j] == 'S'
                        && grid[i + 1][j + 1] == 'A'
                        && grid[i + 2][j + 2] == 'M'
                        && grid[i + 3][j + 3] == 'X')
                {
                    count += 1;
                }
            }
            // diagonal reverse
            if i + 3 < grid.len() && j >= 3 {
                if (grid[i][j] == 'X'
                    && grid[i + 1][j - 1] == 'M'
                    && grid[i + 2][j - 2] == 'A'
                    && grid[i + 3][j - 3] == 'S')
                    || {
                        grid[i][j] == 'S'
                            && grid[i + 1][j - 1] == 'A'
                            && grid[i + 2][j - 2] == 'M'
                            && grid[i + 3][j - 3] == 'X'
                    }
                {
                    count += 1;
                }
            }
        }
    }

    // Also, solve for X pattern of "MAS" in the input
    let mut x_mas_count = 0;
    for i in 0..grid.len() - 2 {
        for j in 0..grid.len() - 2 {
            let possible_list = [
                [(0, 0), (2, 0), (0, 2), (2, 2)],
                [(0, 0), (0, 2), (2, 0), (2, 2)],
                [(2, 0), (2, 2), (0, 0), (0, 2)],
                [(0, 2), (2, 2), (0, 0), (2, 0)],
            ];
            'o: for possible in possible_list {
                if grid[i + possible[0].0][j + possible[0].1] == 'M'
                    && grid[i + possible[1].0][j + possible[1].1] == 'M'
                    && grid[i + possible[2].0][j + possible[2].1] == 'S'
                    && grid[i + possible[3].0][j + possible[3].1] == 'S'
                    && grid[i + 1][j + 1] == 'A'
                {
                    x_mas_count += 1;
                    break 'o;
                }
            }
        }
    }

    println!("Count: {}", count);
    println!("XMAS Count: {}", x_mas_count);
    println!("Elapsed: {:.4}s", now.elapsed().as_secs_f64());
}
