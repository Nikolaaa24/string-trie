use std::io::stdin;
use std::time::Instant;
use string_trie::{Trie, ResultLimit};

fn main() -> ! {
    println!("Dictionary");
    let mut dictionary = Trie::new();

    loop {
        let input = read_input_safe("Use /help for command list: ", "Error while reading input, try again!");
        let command = input.trim().split_whitespace().next().unwrap_or("");
        let arg = input[command.len()..].trim();
        match command
        {
            "/help" => {
                println!("    Available commands:");
                println!("    /help - this page");
                println!("    /exit - exits the program");
                println!("    /credits - shows credits");
                println!("    /search <string> - searches for a string in the dictionary and returns all string matching the prefix string");
                println!("    /add - adds a string to the dictionary")
            }
            "/exit" => {
                println!("    Exiting...");
                std::process::exit(0);
            },
            "/credits" => {
                println!("    Made by Nikola using Rust.");
            },
            "/search" => {
                let start = Instant::now();
                println!("    Searching for words matching: {}", arg);
                let limit = ResultLimit::Limited(10);
                for element in dictionary.search(arg, limit)
                {
                    println!("        {}", element);
                }
                println!("    Search took {:?}", start.elapsed());
            },
            "/add" => {
                let start = Instant::now();
                println!("    Adding word: {}", arg);
                dictionary.insert(arg);
                println!("    Adding took {:?}", start.elapsed());
            },
            "/delete" => {
                let start = Instant::now();
                println!("    Deleting word: {}", arg);
                if dictionary.delete(arg)
                {
                    println!("    Deleted word {} successfully", arg);
                }
                else {
                    println!("    Word {} not found", arg);
                }
                println!("    Deleting took {:?}", start.elapsed());
            },
            _ => {println!("    Invalid command")}
        }
    }
}

// Just a helper func to not have to use match directly
fn read_input_safe(text: &str, error: &str) -> String
{
    let stdin = stdin();
    let mut input = String::new();
    loop {
        println!("{text}");
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(_) => return input,
            Err(_) => {
                println!("{error}");
            }
        }
    }
}