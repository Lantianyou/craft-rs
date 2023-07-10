use crate::scanner::Scanner;

mod scanner;
mod token;
mod ast;

fn main() {
    let mut scanner = Scanner::new("1 + 2");
    scanner.scan_tokens();
}
