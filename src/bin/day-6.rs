use std::{collections::HashSet, env::args, time::Instant};

const INPUT: &str = include_str!("../../day-6-input.txt");
const INPUT_T: &str = include_str!("../../day-6-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let line_len = input.find("\n").unwrap();
    let start_pos = input.find("^").unwrap();
    let start_indice = (start_pos % (line_len + 1), start_pos / (line_len + 1)); // [x, y]
    let mut grid = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(line_len + 1)
        .map(|line| line[0..line_len].to_vec())
        .collect::<Vec<_>>();

    let get_next_pos =
        |cur_pos: (usize, usize), cur_direction: usize| match (cur_direction, cur_pos) {
            (0, (x, y)) if y > 0 => Some((x, y - 1)),
            (1, (x, y)) if x < line_len - 1 => Some((x + 1, y)),
            (2, (x, y)) if y < line_len - 1 => Some((x, y + 1)),
            (3, (x, y)) if x > 0 => Some((x - 1, y)),
            _ => None,
        };
    let get_next_dir = |cur_direction: usize| (cur_direction + 1) % 4;
    let get_next_char = |pos: (usize, usize), grid: &Vec<Vec<char>>| {
        grid.get(pos.1).and_then(|line| line.get(pos.0)).cloned()
    };
    let get_next_pos_and_char =
        |cur_pos: (usize, usize), cur_direction: usize, grid: &Vec<Vec<char>>| {
            let next_pos = get_next_pos(cur_pos, cur_direction);
            let next_char = next_pos.and_then(|pos| get_next_char(pos, grid));
            match (next_pos, next_char) {
                (Some(pos), Some(c)) => Some((pos, c)),
                _ => None,
            }
        };

    // MAP OUT THE POSSIBLE PATHS
    let mut cur_direction = 0; // 0: up, 1: right, 2: down, 3: left
    let mut cur_pos = start_indice;
    grid[cur_pos.1][cur_pos.0] = '*';

    // COUNT THE NUMBER OF OBSTRUCTIONS POSSIBLE
    let mut obstruction_pos_set = HashSet::<(usize, usize)>::new();
    let dir_grid = grid
        .iter()
        .map(|line| line.iter().map(|_| [false; 4]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    while let Some((next_pos, next_char)) = get_next_pos_and_char(cur_pos, cur_direction, &grid) {
        match next_char {
            // Check obstruction only if next position is not visited
            '.' => {
                // Check if possible to place obstruction into next position
                // simulate as if obstruction is placed in next position
                // if loop is detected, then obstruction is possible
                // then continue in the same direction
                grid[next_pos.1][next_pos.0] = '#';
                let mut sim_cur_pos = cur_pos;
                let mut sim_cur_direction = get_next_dir(cur_direction);
                let mut dir_grid = dir_grid.clone();
                dir_grid[cur_pos.1][cur_pos.0][cur_direction] = true;
                'sim: while let Some((sim_next_pos, sim_next_char)) =
                    get_next_pos_and_char(sim_cur_pos, sim_cur_direction, &grid)
                {
                    match sim_next_char {
                        '*' | '.' => {
                            // Check if loop is detected -> visited this position before in the
                            // same direction
                            if dir_grid[sim_next_pos.1][sim_next_pos.0][sim_cur_direction] {
                                obstruction_pos_set.insert(next_pos);
                                break 'sim;
                            }

                            dir_grid[sim_cur_pos.1][sim_cur_pos.0][sim_cur_direction] = true;
                            sim_cur_pos = sim_next_pos;
                        }
                        '#' => {
                            // Change direction
                            sim_cur_direction = get_next_dir(sim_cur_direction);
                        }
                        _ => panic!("Invalid character: {}", sim_next_char),
                    }
                }
                grid[next_pos.1][next_pos.0] = '.';

                // Continue in the same direction, change current position to '*' and move to next
                // position
                grid[cur_pos.1][cur_pos.0] = '*';
                cur_pos = next_pos;
            }
            '*' => {
                // Continue in the same direction, change current position to '*' and move to next
                // position
                grid[cur_pos.1][cur_pos.0] = '*';
                cur_pos = next_pos;
            }
            '#' => {
                // Change direction
                cur_direction = get_next_dir(cur_direction);
            }
            _ => panic!("Invalid character: {}", next_char),
        }
    }

    grid[cur_pos.1][cur_pos.0] = '*';
    println!(
        "Possible distinct positions: {}",
        grid.iter().flatten().filter(|&&c| c == '*').count()
    );
    println!("Obstructions possible: {}", obstruction_pos_set.len());
    println!("Time: {:.4}ms", now.elapsed().as_secs_f64());
}
