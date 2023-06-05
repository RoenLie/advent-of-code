use crate::utils::get_file_contents;
use crate::utils::print_task_and_part;

pub fn task4() {
    part1();
    part2();
}

fn part1() {
    let result = get_file_contents("src/day_4/input.txt".to_string());
    let file_content = result.unwrap_or("".to_string());

    let pairs: Vec<Vec<Vec<i32>>> = file_content
        .split("\r\n")
        .map(|str| {
            str.split(',')
                .map(|range| range.split('-').map(|lim| lim.parse().unwrap()).collect())
                .collect()
        })
        .collect();

    fn is_entry_fully_contained(a: &[i32], b: &[i32]) -> bool {
        a[0] >= b[0] && a[1] <= b[1]
    }

    let mut fully_contained_pairs = 0;

    for pair in pairs {
        //  println!("{:?}", pair);
        let a = &pair[0];
        let b = &pair[1];

        if is_entry_fully_contained(a, b) || is_entry_fully_contained(b, a) {
            fully_contained_pairs += 1;
        }
    }

    print_task_and_part(4, 1);
    println!(
        "pairs where one is fully contained: {}",
        fully_contained_pairs
    );
    println!("\n\n");
}

fn part2() {
    let result = get_file_contents("src/day_4/input.txt".to_string());
    let file_content = result.unwrap_or("".to_string());

    let pairs: Vec<Vec<Vec<i32>>> = file_content
        .split("\r\n")
        .map(|str| {
            str.split(',')
                .map(|range| range.split('-').map(|lim| lim.parse().unwrap()).collect())
                .collect()
        })
        .collect();

    fn is_entry_partially_contained(a: &[i32], b: &[i32]) -> bool {
        a[0] >= b[0] && a[0] <= b[1] || a[1] >= b[0] && a[1] <= b[1]
    }

    let mut fully_contained_pairs = 0;

    for pair in pairs {
        //  println!("{:?}", pair);
        let a = &pair[0];
        let b = &pair[1];

        if is_entry_partially_contained(a, b) || is_entry_partially_contained(b, a) {
            fully_contained_pairs += 1;
        }
    }

    print_task_and_part(4, 2);
    println!(
        "pairs where one is partially contained: {}",
        fully_contained_pairs
    );
    println!("\n\n");
}
