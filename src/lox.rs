use std::io::prelude::*;
use std::path;
use std::io;
use std::fs;
use std::process;
use token::{Token, TokenType};

use scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = scanner::Scanner::new(&source);
        let mut tokens: Vec<Token> = Vec::new();
        match scanner.scan_tokens() {
            Ok(ts) => tokens = ts,
            Err(err) => self.report(err.line(), format!("{}", err)),
        };

        println!("{:?}", tokens);
    }

    pub fn prompt(&mut self) {
        let mut input = String::new();
        let stdin = io::stdin();
        loop {
            print!(">> ");
            io::stdout().flush();
            stdin.lock().read_line(&mut input).unwrap();
            self.run(input.clone());
            input.clear();
            self.had_error = false;
        }
    }

    pub fn runfile(&mut self, path: path::PathBuf) {
        let mut source = String::new();
        let mut file = fs::File::open(path).expect("Failed to open file");

        file.read_to_string(&mut source).expect("Failed to read file");
        self.run(source);

        if self.had_error {
            process::exit(1);
        }
    }

    pub fn report(&mut self, line: usize, message: String) {
        println!("[line {}] Error: {}", line, message);
        self.had_error = true;
    }
}
