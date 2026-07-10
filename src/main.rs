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

fn main() {
    loop {
        let username = get_username();
        let os_name = get_os_name();
        print!("{}@{}@{}> ", SHELL_NAME, username, os_name);
        let input = get_string();

        if input == "exit" {
            break;
        }

        println!("You typed: {}", input);
    }
}
