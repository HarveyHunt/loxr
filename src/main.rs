mod lox;
mod scanner;
mod token;

use std::path;
use std::env;

fn main() {
    let mut lox = lox::Lox::new();
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        lox.prompt();
    } else if args.len() == 2 {
        lox.runfile(path::PathBuf::from(&args[1]));
    } else {
        println!("usage: loxr [script]");
    }
}
