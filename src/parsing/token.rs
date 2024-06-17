use crate::errors::error::*;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    INT(i32),
    FLOAT(f32),
    STRING(String),
    ID(String),
    DEQL,
    EQL,
    LTE,
    LT,
    GTE,
    GT,
    NEQ,
    NOT,
    AND,
    MOD,
    OR,
    SEMI,
    CLN,
    LPR,
    RPR,
    CMA,
    LBR,
    RBR,
    LSQB,
    RSQB,
    PLS,
    MIN,
    MUL,
    DIV,
    DOT,
    EOF
}
pub struct Token {
    pub kind : TokenType,
    pub einfo : ErrorInfo,
}
impl Token {
    pub fn new(kind : TokenType, einfo : ErrorInfo) -> Token {
        Token {
            kind, einfo
        }
    }
    pub fn print_toks(toks : &Vec<Token>) {
        for t in toks {
            match &t.kind {
                TokenType::STRING(s) => println!("STRING: {s}"),
                TokenType::FLOAT(f) => println!("FLOAT: {f}"),
                TokenType::INT(i) => println!("INT: {i}"),
                TokenType::ID(s) => println!("ID: {s}"),
                TokenType::DEQL => println!("DEQL"),
                TokenType::EQL => println!("EQL"),
                TokenType::LTE => println!("LTE"),
                TokenType::LT => println!("LT"),
                TokenType::GTE => println!("GTE"),
                TokenType::GT => println!("GT"),
                TokenType::NEQ => println!("NEQ"),
                TokenType::NOT => println!("NOT"),
                TokenType::AND => println!("AND"),
                TokenType::MOD => println!("MOD"),
                TokenType::OR => println!("OR"),
                TokenType::SEMI => println!("SEMI"),
                TokenType::CLN => println!("CLN"),
                TokenType::LPR => println!("LPR"),
                TokenType::RPR => println!("RPR"),
                TokenType::CMA => println!("CMA"),
                TokenType::LBR => println!("LBR"),
                TokenType::RBR => println!("RBR"),
                TokenType::LSQB => println!("LSQB"),
                TokenType::RSQB => println!("RSQB"),
                TokenType::PLS => println!("PLS"),
                TokenType::MIN => println!("MIN"),
                TokenType::MUL => println!("MUL"),
                TokenType::DIV => println!("DIV"),
                TokenType::DOT => println!("DOT"),
                TokenType::EOF => println!("END OF FILE")
                //_ => println!("UNPRINTABLE TOKEN"),
            }
        }
    }
}
