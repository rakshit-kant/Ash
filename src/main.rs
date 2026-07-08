use std::io::{self, Write};

fn get_string() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read from Stdin");

    input.truncate(input.trim_end().len());

    input
}

fn main() {
    loop {
        print!("rustsh> ");
        let input = get_string();

        if input == "exit" {
            break;
        }

        println!("You typed: {}", input);
    }
}
