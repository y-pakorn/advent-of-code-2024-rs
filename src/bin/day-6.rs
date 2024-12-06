use std::borrow::BorrowMut;

const input: &str = include_str!("../../day-6-input.txt");

fn main() {
    let line_len = input.find("\n").unwrap();
    let start_pos = input.find("^").unwrap();
    let start_indice = (start_pos % (line_len + 1), start_pos / (line_len + 1)); // [x, y]
    let mut grid = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(line_len + 1)
        .map(|line| line[0..line_len].to_vec())
        .collect::<Vec<_>>();
    let mut cur_direction = 0; // 0: up, 1: right, 2: down, 3: left
    let mut cur_pos = start_indice;
    loop {
        let next_pos = match cur_direction {
            0 => (cur_pos.0, cur_pos.1 - 1),
            1 => (cur_pos.0 + 1, cur_pos.1),
            2 => (cur_pos.0, cur_pos.1 + 1),
            3 => (cur_pos.0 - 1, cur_pos.1),
            _ => panic!("Invalid direction"),
        };
        let next_char = grid
            .get(next_pos.1)
            .and_then(|line| line.get(next_pos.0))
            .cloned();
        match next_char {
            Some('.') | Some('*') => {
                // Continue in the same direction, change current position to '*' and move to next
                // position
                grid[cur_pos.1][cur_pos.0] = '*';
                cur_pos = next_pos;
                println!("Next position: {:?}", cur_pos);
            }
            Some('#') => {
                // Change direction
                cur_direction = (cur_direction + 1) % 4;
                println!("Change direction to: {}", cur_direction);
            }
            _ => {
                grid[cur_pos.1][cur_pos.0] = '*';
                println!("{:?}", grid);
                println!("End of the line");
                println!(
                    "Number of *: {}",
                    grid.iter().flatten().filter(|&&c| c == '*').count()
                );
                break;
            }
        }
    }
}
