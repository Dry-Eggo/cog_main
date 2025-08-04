#![allow(unused)]

pub enum Token {
    Func, Let,
    
    Identifier(String),
    Integer(String),

    Eq, Add, Sub, Mul, Div,

    Colon, SemiColon, Comma, Coleq,
    OParen, CParen, OBrace, CBrace,
}
