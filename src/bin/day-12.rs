use std::{collections::HashSet, env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-12-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-12-input-t.txt");

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

    fn visit_and_check_discounted(
        val: char,
        x: usize,
        y: usize,
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> (usize, usize, usize) {
        let mut area = 0;
        let mut sides = 0;
        let mut stack: Vec<(usize, usize)> = vec![(x, y)];
        let mut prev_sides: HashSet<(isize, isize, char)> = HashSet::new();

        let directions = [(0, 1, 'a'), (1, 0, 'b'), (0, -1, 'c'), (-1, 0, 'd')];

        while let Some((x, y)) = stack.pop() {
            if visited[x][y] {
                continue;
            }

            visited[x][y] = true;
            area += 1;

            for &(dx, dy, is_vert) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < grid[0].len() as isize && ny >= 0 && ny < grid.len() as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if grid[ny][nx] == val {
                        if !visited[nx][ny] {
                            stack.push((nx, ny));
                        }
                    } else {
                        prev_sides.insert((nx as isize, ny as isize, is_vert));
                    }
                } else {
                    prev_sides.insert((nx, ny, is_vert));
                }
            }
        }

        let perimeters = prev_sides.len();

        while let Some(&(x, y, is_vert)) = prev_sides.iter().next() {
            prev_sides.remove(&(x, y, is_vert));
            sides += 1;
            'o: for direction in directions
                .iter()
                .filter(|(_, _, dis_vert)| *dis_vert != is_vert)
            {
                let mut i = 1;
                loop {
                    let nx = x + direction.0 * i;
                    let ny = y + direction.1 * i;

                    if !prev_sides.remove(&(nx, ny, is_vert)) {
                        continue 'o;
                    }

                    i += 1;
                }
            }
        }

        (area, perimeters, sides)
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut cost = 0;
    let mut discounted_cost = 0;

    for y in 0..visited.len() {
        for x in 0..visited[y].len() {
            if !visited[x][y] {
                let (a, p, s) = visit_and_check_discounted(grid[y][x], x, y, &grid, &mut visited);
                cost += a * p;
                discounted_cost += a * s;
            }
        }
    }

    println!("Cost: {}", cost);
    println!("Discounted Cost: {}", discounted_cost);
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
