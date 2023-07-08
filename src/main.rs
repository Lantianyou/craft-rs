use crate::scanner::Scanner;

mod scanner;
mod token;
mod ast;

fn main() {
    let scanner = Scanner::new("1 + 2");
}
