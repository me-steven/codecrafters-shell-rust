#[allow(unused_imports)]
use std::io::{self, Write};

fn find_in_path(c: &str) {
    if which::which(c.trim()).is_ok() {
        println!("{}: is a shell builtin", c.trim());
    } else {
        println!("{}: not found", c.trim());
    }
}

fn command_does_not_exists(c: String) {
    println!("{}: command not found", c.trim());
}

fn command_echo(c: String) {
    print!("{}", &c[5..]);
}

fn command_type(c: String) {
    let given: &str = &c[5..].trim();

    match given {
        "type" => println!("{} is a shell builtin", given),
        "echo" => println!("{} is a shell builtin", given),
        "exit" => println!("{} is a shell builtin", given), 
        _ => println!("{}: not found", given.trim())
    }
}

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        
        match command.as_str().split_whitespace().next().unwrap_or("").trim() {
            "type" => command_type(command),
            "echo" => command_echo(command),
            "exit" => break,
            // _ => command_does_not_exists(command)
            _ => command_does_not_exists(command)
        }
    }
}
