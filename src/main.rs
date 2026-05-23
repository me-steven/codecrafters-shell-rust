#[allow(unused_imports)]
use std::io::{self, Write};

fn command_does_not_exists(c: String) {
    println!("{}: command not found", c.trim());
}

fn command_echo(c: String) {
    print!("{}", &c[5..]);
}

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        match command.as_str().split_whitespace().next().unwrap_or("") {
            "echo" => command_echo(command),
            "exit" => break,
            _ => command_does_not_exists(command)
        }
    }
}
