#[allow(unused_imports)]
use std::io::{self, Write};

fn command_not_exists(c: String) {
    println!("{}: command not found", c.trim());
}
fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        match command.as_str().split_whitespace().next().unwrap_or("") {
            "echo" => print!("{}", &command[5..]),
            "exit" => break,
            // _ => println!("{}: command not found", command.trim())
            _ => command_not_exists(command)
        }
    }
}
