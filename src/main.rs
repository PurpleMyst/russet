#[macro_use] extern crate lazy_static;
extern crate regex;

mod tokenizer;
mod parser;

fn main() {
    let code = "1 + 2\n3\t+  4";
    let ast = parser::parse(&code);

    println!("{:?}", ast);
}
