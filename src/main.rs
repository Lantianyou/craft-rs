use crate::scanner::Scanner;

mod ast;
mod scanner;
mod token;
// mod expr;

fn main() {
    let mut scanner = Scanner::new("1 + 2");
    scanner.scan_tokens();
    println!("{:?}", scanner);
    ast::main();
}
