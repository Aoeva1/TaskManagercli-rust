use std::io;
use colored::*;  // for colored output
use chrono::*;
// Define the task management struct
struct TaskManage {
    name: String,
    complete: bool,
    priority: u128,

}

impl TaskManage {
    // Method to check the task details
    fn check(&self) {
        println!(
            "Task: {}, Priority: {}, Completed: {}",
            self.name.green().bold(),
            self.priority.to_string().red(),
            self.complete.to_string().bright_blue()
        );
    }

    // Constructor to create a new task
    fn new(name: String, priority: u128) -> Self {
        TaskManage {
            name,
            complete: false, // Default to incomplete
            priority,
        }
    }
}

fn main() {
    // Create a mutable String to store the user input







    let banner = r#"
(_  _) /__\  / __)( )/ )  (  \/  )  /__\  ( \( )  /__\  / __)( ___)(  _ \   / __)(  )  (_  _)
  )(  /(__)\ \__ \ )  (    )    (  /(__)\  )  (  /(__)\( (_-. )__)  )   /  ( (__  )(__  _)(_
 (__)(__)(__)(___/(_)\_)  (_/\/\_)(__)(__)(_)\_)(__)(__)\___/(____)(_)\_)   \___)(____)(____)

    "#;
    println!(", {}",  banner.red().bold());

let one  =  ("1. make new task \n");
let two = ("2. mark task as complete task \n");
let three = ("3. delete task  \n");
let four = ("4. save and quit \n");
    println!(", {}",  one.green().bold());
    println!(", {}", two.bright_blue().bold());
    println!(", {}", three.bright_magenta().bold());
    println!(", {}", four.bright_yellow().bold());

    let mut y = 10;

    // Prompt the user for input


    // Read input from stdin and handle potential errors
    io::stdin().read_line(&mut y.to_string()).expect("Failed to read line");

    // Trim the newline character at the end and print the input


    // Prompt the user for input

}
