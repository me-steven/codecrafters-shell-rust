#[allow(unused_imports)]
use std::io::{self, Write};

use is_executable::IsExecutable;

fn find_in_path(c: &str) {
    let binding: String = std::env::var("PATH").unwrap();
    let paths: std::env::SplitPaths<'_> = std::env::split_paths(binding.as_str());

    for path in paths {
        let full_path: std::path::PathBuf = path.join(c);
        if full_path.exists() && full_path.is_executable(){
            println!("{} is {}", c.trim(), full_path.display());
            return;
        }
    }

    println!("{}: not found", c.trim());
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
        _ => find_in_path(given)
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
            _ => command_does_not_exists(command)
        }
    }
}
