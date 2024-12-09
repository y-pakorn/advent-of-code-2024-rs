use std::{env::args, time::Instant};

const INPUT: &str = include_str!("../../inputs/day-9-input.txt");
const INPUT_T: &str = include_str!("../../inputs/day-9-input-t.txt");

fn main() {
    let now = Instant::now();
    let input = match args().collect::<Vec<_>>().contains(&String::from("--test")) {
        true => INPUT_T,
        false => INPUT,
    };

    let layout = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u8>().expect("Invalid input"))
        .collect::<Vec<_>>();

    let (mut chars, _) = layout.into_iter().enumerate().fold(
        (vec![], 0usize),
        |(mut cur_s, mut cur_id): (Vec<Option<usize>>, usize), (i, c)| {
            match i % 2 == 0 {
                true => {
                    (0..c).for_each(|_| cur_s.push(Some(cur_id)));
                    cur_id += 1;
                }
                false => {
                    (0..c).for_each(|_| cur_s.push(None));
                }
            }
            (cur_s, cur_id)
        },
    );
    let mut wf_chars = chars.clone();

    let mut free_idx = chars
        .iter()
        .position(|&c| c.is_none())
        .unwrap_or(chars.len() - 1);
    let mut last_pos_idx = chars.len() - 1;
    while free_idx < last_pos_idx {
        if chars[last_pos_idx].is_some() {
            chars.swap(free_idx, last_pos_idx);
        }
        last_pos_idx -= 1;
        while chars[free_idx].is_some() {
            free_idx += 1;
        }
    }

    let sum = chars
        .iter()
        .take_while(|&&c| c.is_some())
        .filter_map(|&c| c)
        .enumerate()
        .map(|(i, c)| i * c)
        .sum::<usize>();

    let mut last_idx = wf_chars.len() - 1;
    let first_none_idx = wf_chars
        .iter()
        .position(|&c| c.is_none())
        .unwrap_or(last_idx);
    while last_idx > first_none_idx {
        match wf_chars[last_idx] {
            None => {
                last_idx -= 1;
            }
            Some(last_char) => {
                let last_char_end = last_idx;
                let last_char_len = wf_chars[..last_idx + 1]
                    .iter()
                    .rev()
                    .take_while(|&&c| c == Some(last_char))
                    .count();
                let last_char_start = last_char_end - last_char_len + 1;

                // search for slice of none that size = last_char_len
                let mut current_search_index = 0;
                'o: while current_search_index < last_idx - last_char_len {
                    let mut found = 0;
                    if last_char_len > 1
                        && !wf_chars[current_search_index + last_char_len - 1].is_none()
                    {
                        current_search_index += last_char_len;
                        continue;
                    }
                    for i in current_search_index..current_search_index + last_char_len {
                        if wf_chars[i].is_none() {
                            found += 1;
                        } else {
                            current_search_index = i + 1;
                            continue 'o;
                        }
                    }
                    if found == last_char_len {
                        for i in 0..last_char_len {
                            wf_chars.swap(last_char_start + i, current_search_index + i);
                        }
                        break 'o;
                    }
                }

                last_idx = last_char_start - 1;
            }
        }
    }

    let wf_sum = wf_chars
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| c.map(|v| i * v))
        .sum::<usize>();

    println!("Sum first part: {}", sum);
    println!("Sum second part: {}", wf_sum);
    println!("Time: {:.4}s", now.elapsed().as_secs_f64());
}
