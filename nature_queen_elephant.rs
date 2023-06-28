extern crate rand;

use std::io::{stdin, stdout, Write};
use rand::Rng;

fn main() {
    println!("Welcome to The Volunteer Project!");
    println!("Please enter your name:");

    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("Hello, {}! Welcome to the project!", name);

    println!("Please enter the length of the project (in days):");
    let mut length = String::new();
    stdin().read_line(&mut length).expect("Failed to read line");
    let length: u32 = match length.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    println!("Please enter your estimated daily work hours:");
    let mut hours = String::new();
    stdin().read_line(&mut hours).expect("Failed to read line");
    let hours: u32 = match hours.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    let mut total_hours = length * hours;
    let mut total_tasks = 0;
    let mut task_hours;

    // Generate tasks
    while total_hours > 0 {
        print!("Task hours for task {}: ", total_tasks);
        stdout().flush().expect("Failed to flush");

        let mut task_hours_input = String::new();
        stdin().read_line(&mut task_hours_input).expect("Failed to read line");
        task_hours = match task_hours_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                return;
            }
        };

        if task_hours > total_hours {
            println!("Task hours cannot exceed the total length of the project! Please enter a valid number.");
            continue;
        }

        total_tasks += 1;
        total_hours -= task_hours;
    }

    println!("Done! You can now start working on the project.");
    println!("You have {} tasks to complete with {} hours of work each.", total_tasks, task_hours);

    let mut daily_work_hours = vec![];
    let mut current_task = 0;

    // Generate a random work hours distribution
for _ in 0..length {
    let work_hours = rand::thread_rng().gen_range(hours - 4, hours + 4);
    daily_work_hours.push(work_hours);
    current_task += 1;
    if current_task > total_tasks {
        break;
    }
}

println!("Suggested Work Hours Distribution:");
let mut total_daily_hours: u32 = 0;
for (i, daily_hours) in daily_work_hours.iter().enumerate() {
    println!("Day {}: {} hours", i + 1, daily_hours);
    total_daily_hours += daily_hours;
}

println!("Total project work hours: {}", total_daily_hours);
}