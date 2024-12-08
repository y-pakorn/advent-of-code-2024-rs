use std::{
    collections::{HashMap, HashSet},
    env::args,
    time::Instant,
};

const INPUT: &str = include_str!("../../inputs/day-8-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-8-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid_x_size = grid[0].len();
    let grid_y_size = grid.len();

    let mut antinodes = HashSet::<(usize, usize)>::new();
    let mut inline_antinodes = HashSet::<(usize, usize)>::new();
    let mut annetas = HashMap::<char, Vec<(usize, usize)>>::new();

    for y in 0..grid_y_size {
        for x in 0..grid_x_size {
            match grid[y][x] {
                '.' => continue,
                c => {
                    annetas.entry(c).or_insert(Vec::new()).push((x, y));
                }
            }
        }
    }

    for (_, nodes) in annetas.iter() {
        match nodes.len() {
            0 | 1 => continue,
            _ => {
                let mut cur_index = 0;
                while cur_index < nodes.len() {
                    for next_node_index in cur_index..nodes.len() {
                        if cur_index == next_node_index {
                            continue;
                        }
                        let (cur_node_x, cur_node_y) = nodes[cur_index];
                        let (next_node_x, next_node_y) = nodes[next_node_index];

                        let dx = next_node_x as i32 - cur_node_x as i32;
                        let dy = next_node_y as i32 - cur_node_y as i32;

                        let mut mf = 1;
                        loop {
                            let antinode_x = cur_node_x as i32 + dx * mf;
                            let antinode_y = cur_node_y as i32 + dy * mf;
                            if (antinode_x < 0 || antinode_x >= grid_x_size as i32)
                                || (antinode_y < 0 || antinode_y >= grid_y_size as i32)
                            {
                                break;
                            }
                            inline_antinodes.insert((antinode_x as usize, antinode_y as usize));
                            if mf == 2 {
                                antinodes.insert((antinode_x as usize, antinode_y as usize));
                            }
                            mf += 1;
                        }
                        let mut ms = 1;
                        loop {
                            let antinode_x = next_node_x as i32 - dx * ms;
                            let antinode_y = next_node_y as i32 - dy * ms;
                            if (antinode_x < 0 || antinode_x >= grid_x_size as i32)
                                || (antinode_y < 0 || antinode_y >= grid_y_size as i32)
                            {
                                break;
                            }
                            inline_antinodes.insert((antinode_x as usize, antinode_y as usize));
                            if ms == 2 {
                                antinodes.insert((antinode_x as usize, antinode_y as usize));
                            }
                            ms += 1;
                        }
                    }
                    cur_index += 1;
                }
            }
        }
    }

    println!("Antinodes {:?}", antinodes.len());
    println!("Inline Antinodes {:?}", inline_antinodes.len());
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
