#[macro_use] extern crate lazy_static;
extern crate regex;

mod tokenizer;

fn main() {
    let code = "1 + 2\n3\t+  4";
    let tokens = tokenizer::Tokenizer::new(&code);

    tokens.for_each(|token| println!("{:?}", token));
}
