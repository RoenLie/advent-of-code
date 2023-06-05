use crate::utils::get_file_contents;
use crate::utils::print_task_and_part;

pub fn task2() {
    part1();
    part2();
}

enum RPSType {
    Rock,
    Paper,
    Scissor,
}

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSOR: i32 = 3;

fn char_to_enum_type(char: &str) -> Result<RPSType, String> {
    match char {
        "A" => Ok(RPSType::Rock),
        "B" => Ok(RPSType::Paper),
        "C" => Ok(RPSType::Scissor),
        "Y" => Ok(RPSType::Paper),
        "X" => Ok(RPSType::Rock),
        "Z" => Ok(RPSType::Scissor),
        _ => Err(String::from("Invalid character")),
    }
}

fn score1(a: &RPSType, b: &RPSType) -> i32 {
    match (a, b) {
        (RPSType::Rock, RPSType::Rock) => ROCK + DRAW,
        (RPSType::Rock, RPSType::Paper) => PAPER + WIN,
        (RPSType::Rock, RPSType::Scissor) => SCISSOR + LOSS,
        //
        (RPSType::Paper, RPSType::Rock) => ROCK + LOSS,
        (RPSType::Paper, RPSType::Paper) => PAPER + DRAW,
        (RPSType::Paper, RPSType::Scissor) => SCISSOR + WIN,
        //
        (RPSType::Scissor, RPSType::Rock) => ROCK + WIN,
        (RPSType::Scissor, RPSType::Paper) => PAPER + LOSS,
        (RPSType::Scissor, RPSType::Scissor) => SCISSOR + DRAW,
    }
}

fn score2(a: &RPSType, b: &str) -> i32 {
    match (a, b) {
        (RPSType::Rock, "Y") => ROCK + DRAW,
        (RPSType::Rock, "Z") => PAPER + WIN,
        (RPSType::Rock, "X") => SCISSOR + LOSS,
        //
        (RPSType::Paper, "X") => ROCK + LOSS,
        (RPSType::Paper, "Y") => PAPER + DRAW,
        (RPSType::Paper, "Z") => SCISSOR + WIN,
        //
        (RPSType::Scissor, "Z") => ROCK + WIN,
        (RPSType::Scissor, "X") => PAPER + LOSS,
        (RPSType::Scissor, "Y") => SCISSOR + DRAW,
        _ => unimplemented!(),
    }
}

fn part1() {
    let result = get_file_contents("src/day_2/input.txt".to_string());
    let file_content = result.unwrap_or("".to_string());

    let matches = file_content.split('\n');

    let mut total_score = 0;

    for round in matches {
        let moves: Vec<&str> = round.split_whitespace().collect();
        let enemy_char = moves[0];
        let player_char = moves[1];

        let enemy_move = char_to_enum_type(enemy_char).unwrap();
        let player_move = char_to_enum_type(player_char).unwrap();

        let round_score = score1(&enemy_move, &player_move);
        total_score += round_score;
    }

    print_task_and_part(2, 1);
    println!("total score: {}", total_score);
    println!("\n\n");
}

fn part2() {
    let result = get_file_contents("src/day_2/input.txt".to_string());
    let file_content = result.unwrap_or("".to_string());

    let matches = file_content.split('\n');

    let mut total_score = 0;

    for round in matches {
        let moves: Vec<&str> = round.split_whitespace().collect();
        let enemy_char = moves[0];
        let player_char = moves[1];

        let enemy_move = char_to_enum_type(enemy_char).unwrap();

        let round_score = score2(&enemy_move, player_char);
        total_score += round_score;
    }

    print_task_and_part(2, 2);
    println!("total score: {}", total_score);
    println!("\n\n");
}
