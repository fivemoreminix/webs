use std::iter::Peekable;
use std::str::Chars;

enum Token {
    Let,
    Eq,
    Print,
    Is,
    ID(String),
    Str(String),
}

fn main() {
    println!("Hello, world!");
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();

    let mut tokens = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            _ => {}
        }
    }

    tokens
}
