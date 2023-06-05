use crate::utils::get_file_contents;
use crate::utils::print_task_and_part;

pub fn task1() {
    part1();
    part2();
}

pub fn part1() {
    let file_contents =
        get_file_contents("src/day_1/input.txt".to_string()).unwrap_or("".to_string());

    let buckets: Vec<&str> = file_contents.split("\r\n\r\n").collect();

    let mut highest_calories = 0;
    for bucket in &buckets {
        let mut calories: i32 = 0;
        let parts: Vec<&str> = bucket.split("\r\n").collect();
        for cals in &parts {
            let num = cals.parse().unwrap_or(0);
            calories += num;
        }

        if calories > highest_calories {
            highest_calories = calories;
        }
    }

    print_task_and_part(1, 1);
    println!("Highest amount of calories is! {}", highest_calories);
    println!("\n\n");
}

pub fn part2() {
    let file_contents =
        get_file_contents("src/day_1/input.txt".to_string()).unwrap_or("".to_string());

    let buckets: Vec<&str> = file_contents.split("\r\n\r\n").collect();

    let mut top_three: [i32; 3] = [0, 0, 0];

    fn insert_calories(mut top_three: [i32; 3], calories: i32) -> [i32; 3] {
        if calories > top_three[2] {
            let old = top_three[2];
            top_three[2] = calories;

            top_three = insert_calories(top_three, old);
        } else if calories > top_three[1] {
            let old = top_three[1];
            top_three[1] = calories;

            top_three = insert_calories(top_three, old);
        } else if calories > top_three[0] {
            let old = top_three[0];
            top_three[0] = calories;

            top_three = insert_calories(top_three, old);
        }

        top_three
    }

    for bucket in &buckets {
        let mut calories: i32 = 0;
        let parts: Vec<&str> = bucket.split("\r\n").collect();
        for cals in &parts {
            let num = cals.parse().unwrap_or(0);
            calories += num;
        }

        top_three = insert_calories(top_three, calories);
    }

    let sum: i32 = top_three.iter().sum();

    print_task_and_part(1, 2);
    println!("Top Three calories are: {:?}", top_three);
    println!("Total top three is: {}", sum);
    println!("\n\n");
}
