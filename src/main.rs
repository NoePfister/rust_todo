use std::{io,process};

fn main() {
    'command: loop {
        clear();
        println!("TODO Manager!");
        println!("Please enter your command:");
        println!("q -> Quit application");
        println!("s -> Show Todo's");
        println!("n -> New Todo");

        let mut command = String::new();

        match io::stdin().read_line(&mut command) {
            Ok(_) => (),
            Err(_) => {
                println!("Invalid command");
                continue 'command;
            }
        }

        command = command.trim().to_lowercase();

        match command.as_str() {
            "q" => quit(),
            "s" => show(),
            "n" => new(),
            _ => {
                println!("Invalid command");
                continue 'command;
            }
        }
    }
}

fn quit() {
    process::exit(0);
}

fn show() {}

fn new() {}

fn clear() {
    print!("{}[2J", 27 as char);
}
