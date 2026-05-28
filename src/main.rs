#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use is_executable::IsExecutable;
use std::process::Command;

type CommandFn = fn(args: &mut dyn Iterator<Item = &str>) -> bool;

const COMMAND_MAP: [(&str, CommandFn); 3] = [
    ("type", command_type),
    ("echo", command_echo),
    ("exit", command_exit)
];

fn find_path(c: &str) -> Option<String> {
    if let Some(path_var) = env::var_os("PATH") {
        for path_dir in env::split_paths(&path_var) {
            let full_path = path_dir.join(c);
            if full_path.is_executable() {
                return Some(full_path.to_string_lossy().into_owned());
            }
    
            if cfg!(windows) {
                if let Some(pathext_var) = env::var_os("PATHEXT") {
                    for ext in env::split_paths(&pathext_var) {
                        let mut path_with_ext = full_path.clone();
                        let ext_str = ext.to_string_lossy();
                        path_with_ext.set_extension(ext_str.trim_start_matches('.'));
                        if path_with_ext.is_executable() {
                            return Some(path_with_ext.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }

    None
}

fn command_not_found(args: &mut dyn Iterator<Item = &str>) {
    let tokens: Vec<&str> = args.collect();
    let c = tokens[0];

    if find_path(c).is_none() {
        println!("{}: command not found", c);
        return;
    }

    Command::new(c)
    .args(&tokens[1..])
    .status()
    .expect("Failed to execute command");


}

fn command_exit(_args: &mut dyn Iterator<Item = &str>) -> bool{
    std::process::exit(0);
}

fn command_echo(args: &mut dyn Iterator<Item = &str>) -> bool{
    let mut output = String::new();

    loop {
        let next: Option<&str> = args.next();
        if next == None {
            break
        }

        output.push_str(next.unwrap());
        output.push_str(" ");
    }

    println!("{}", output.trim());

    true
}

fn command_pwd(args: &mut dyn Iterator<Item = &str>) -> bool{
    let status = env::current_dir();
    
    if status.is_ok() {
        println!("{}", status.unwrap().display());
        return true;
    }

    eprintln!("Error getting current directory: {}", status.err().unwrap());
    return false;
    
 

}

fn command_type(args: &mut dyn Iterator<Item = &str>) -> bool{
    let command = args.next();
    
    if command == None {
        return false;
    }

    let command_str = command.unwrap();


    if !search_commands(command_str).is_none() {
        println!("{} is a shell builtin", command_str);
        return true;
    } else if let Some(lookup) = find_path(command_str) {
        println!("{} is {}", command_str, lookup);
        return false;
    } else {
        println!("{}: not found", command_str);
        return false;
    }
}

fn search_commands(c: &str) -> Option<&'static (&'static str, CommandFn)>{
    COMMAND_MAP.iter().find(|&&(name, _)| name == c)
}

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let components: std::str::SplitWhitespace<'_>  = input.trim().split_whitespace();
        let tokens: Vec<&str> = components.collect();

        if tokens.is_empty() {
            continue;
        }

        let command_str = tokens[0];
        let returned_commands = search_commands(command_str); 
        
        match returned_commands {
            Some((_name, func)) => {
                let mut args = tokens[1..].iter().map(|s| *s);
                func(&mut args);
            }

            None => {
                let mut args = tokens.iter().map(|s| *s);
                command_not_found(&mut args);
            }
        }
    }
}
