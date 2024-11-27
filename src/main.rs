use colored::*; // for colored output
use serde::{Deserialize, Serialize};
use std::{fs, io};

use std::io::{Read, Write};
use serde_json::{Value, json};


use std::fs::{OpenOptions, File};
use std::process::exit;

#[derive(Serialize, Deserialize, Debug)]
struct TaskManage {
    name: String,
    complete: bool,
    priority: Vec<i32>,
}


    impl TaskManage {
    // Check the task details
    fn check(&self) {
        println!(
            "Task: {}, Priority: {:?}, Completed: {}",
            self.name.green().bold(),
            self.priority,
            self.complete.to_string().bright_blue()
        );
    }

    // Create a new task
    fn new(name: String, priority: Vec<i32>) -> Self {
        TaskManage {
            name,
            complete: false,
            priority,
        }
    }
}



fn load_tasks(file_path: &str) -> Vec<TaskManage> {
    let content = fs::read_to_string(file_path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
}
// Save tasks to a file
fn save_tasks(task: &Vec<TaskManage>, file_path: &str) {
    let data = serde_json::to_string_pretty(task).expect("Failed to serialize task");
    fs::write(file_path, data).expect("Failed to write to file");
    println!("{}", "Tasks saved successfully!".green());
}


fn main() {
    let file_path = "tasks.json";

    let mut tasks = load_tasks(file_path);   // Load existing finances or create new


    loop {
        let banner = r#"

(_  _) /__\  / __)( )/ )  (  \/  )  /__\  ( \( )  /__\  / __)( ___)(  _ \   / __)(  )  (_  _)
  )(  /(__)\ \__ \ )  (    )    (  /(__)\  )  (  /(__)\( (_-. )__)  )   /  ( (__  )(__  _)(_
 (__)(__)(__)(___/(_)\_)  (_/\/\_)(__)(__)(_)\_)(__)(__)\___/(____)(_)\_)   \___)(____)(____)
    "#;
        println!("{}", banner.red().bold());

        println!("{}", "1. Create a new task".green().bold());
        println!("{}", "2. Mark a task as complete".bright_blue().bold());
        println!("{}", "3. Delete a task".bright_magenta().bold());
        println!("{}", "4. Save and quit".bright_yellow().bold());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can't read line");
        let input = input.trim();


        match input {
            "1" => {
                println!("{}", "Enter task name:".cyan());
                let mut task_name = String::new();
                io::stdin()
                    .read_line(&mut task_name)
                    .expect("Failed to read task name");
                let task_name = task_name.trim().to_string();

                println!("{}", "Enter task priority (comma-separated numbers):".cyan());
                let mut priority_input = String::new();
                io::stdin()
                    .read_line(&mut priority_input)
                    .expect("Failed to read priority");
                let priority: Vec<i32> = priority_input
                    .trim()
                    .split(',')
                    .filter_map(|p| p.trim().parse::<i32>().ok())
                    .collect();

                let new_task = TaskManage::new(task_name.clone(), priority);

                println!("{} was added to the task list", new_task.name.green().bold());
                tasks.push(new_task);

            },
            "2" => {

            },
            "3" => {

            },
            "4" => {
                save_tasks(&tasks, &file_path);
                exit(0)
            }
            _ => {}
        }
    }
}

























































