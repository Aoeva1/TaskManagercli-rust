


use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use colored::*;


struct taskmanage{
    name:String,
    complete:bool,
    priority:u128,

}
impl taskmanage {


    fn check(&self) {
        println!(
            "Tasks are {}, priority is {}, completed is {}",
            self.name.green().bold(),
            self.priority.to_string().red(),
            self.complete.to_string().bright_blue())
    }

    fn new(name: String, priority: u128) -> Self {
        taskmanage {
            name,
            complete: false, // Default to incomplete
            priority,
        }
    }
}


fn main() {
    let mut taskmanage = taskmanage {
        name: "cleaning".to_string(),
        priority: 1,
        complete: true,

    };
    taskmanage.check();
    let styled_text = "hi".red().bold().underline();
    println!("{}", styled_text);
    println!("\x1b[34mThis is blue\x1b[0m");


}