use super::tokenizer::Tokens;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AstValue {
    String(String),
    Number(isize),

    Addition(Box<AstValue>, Box<AstValue>),
    Subtraction(Box<AstValue>, Box<AstValue>),
    Multiplication(Box<AstValue>, Box<AstValue>),
    Division(Box<AstValue>, Box<AstValue>),

    /* TODO: Pattern matches? */
    Assignment(String, Box<AstValue>),
}

pub fn parse(code: &str) -> AstValue {
    let _tokens = Tokens::tokenize(code);

    AstValue::String("TODO".to_owned())
}
