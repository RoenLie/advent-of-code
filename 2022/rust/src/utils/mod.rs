use std::env;
use std::fs::File;
use std::io::{self, Read};

pub fn get_file_contents(path: String) -> io::Result<String> {
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join(path);

    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn print_task_and_part(task: i32, part: i32) {
    println!("TASK {} - Part{}", task, part);
    println!("----------------------------------------------------");
}
