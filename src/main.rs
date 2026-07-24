use std::fs;
use std::io::{self, Write};

const SHELL_NAME: &str = "ASH";

fn get_string() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read from Stdin");

    input.truncate(input.trim_end().len());

    input
}

fn get_username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
}

fn get_os_name() -> String {
    let contents = fs::read_to_string("/etc/os-release").expect("Failed to Read os-release");

    for line in contents.lines() {
        if line.starts_with("ID=") {
            return line
                .strip_prefix("ID=")
                .unwrap()
                .trim_matches('"')
                .to_string();
        }
    }

    "Unknown".to_string()
}

fn builtin_pwd() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("Error: {err}"),
    }
}

fn builtin_cd(path_name: &str) {
    // TODO: Implement cd
    let _ = path_name;
}

fn dispatch(tokens: &[String]) {
    match tokens.first().map(String::as_str) {
        Some("pwd") => builtin_pwd(),
        Some("cmd") => eprintln!("What you wrote is wrong"),
        Some("cd") => {
            if let Some(path) = tokens.get(1) {
                builtin_cd(path);
            } else {
                eprintln!("cd: missing operand");
            }
        }
        None => {}
        _ => {}
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for current_character in input.chars() {
        if current_character.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(current_character);
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

fn main() {
    loop {
        let username = get_username();
        let os_name = get_os_name();
        print!("{}@{}@{}> ", SHELL_NAME, username, os_name);
        let input = get_string();

        let tokens = tokenize(&input);
        dispatch(&tokens);
    }
}

// TODO: Initialize the path_name variable
// TODO: Scan for the Directories and Files in the Current Directory
// TODO: Finish the fn builtin_cd()
