use std::collections::HashSet;

use crate::utils::get_file_contents;
use crate::utils::print_task_and_part;

pub fn task3() {
    part1();
    part2();
}

fn get_char_priority(char: char) -> u32 {
    let mut ascii = char as u32;

    if char.is_lowercase() {
        ascii -= 96;
    } else {
        ascii -= 38;
    }

    ascii
}

fn part1() {
    let result = get_file_contents("src/day_3/input.txt".to_string());
    let file_content = result.unwrap_or("".to_string());

    let lines: Vec<&str> = file_content.split("\r\n").collect();

    let mut duplicate_char_list: Vec<char> = Vec::with_capacity(lines.len());

    for line in lines {
        let line_length = line.len();
        let split_str = line.split_at(line_length / 2);
        let first_half = split_str.0;
        let second_half = split_str.1;

        let second_chars: HashSet<char> = second_half.chars().collect();
        for char in first_half.chars() {
            if second_chars.contains(&char) {
                duplicate_char_list.push(char);
                break;
            }
        }
    }

    let mut total_priority = 0;

    for char in duplicate_char_list {
        let mut ascii = char as u32;

        if char.is_lowercase() {
            ascii -= 96;
        } else {
            ascii -= 38;
        }

        total_priority += ascii;
    }

    print_task_and_part(3, 1);
    println!("total priority: {}", total_priority);
    println!("\n\n");
}

fn part2() {
    let result = get_file_contents("src/day_3/input.txt".to_string());
    let file_content = result.unwrap_or("".to_string());

    let lines: Vec<&str> = file_content.split("\r\n").collect();
    let groups: Vec<Vec<&str>> = lines.chunks_exact(3).map(|chunk| chunk.to_vec()).collect();

    let mut total_priority = 0;
    for group in groups {
        if let Some(first_three) = group.get(0..3) {
            let first = first_three[0];
            let second: HashSet<char> = first_three[1].chars().collect();
            let third: HashSet<char> = first_three[2].chars().collect();

            for char in first.chars() {
                if second.contains(&char) && third.contains(&char) {
                    total_priority += get_char_priority(char);
                    break;
                }
            }
        }
    }

    print_task_and_part(3, 2);
    println!("total priority: {}", total_priority);
    println!("\n\n");
}
