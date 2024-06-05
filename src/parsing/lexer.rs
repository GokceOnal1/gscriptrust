use super::token::*;
use std::fs;
use crate::errors::error::*;


pub struct Lexer<'a> {
    filename : String,
    pub tokens : Vec<Token>,
    source :  Vec<char>,
    sourcelines : Vec<String>,
    curri : usize,
    currline : usize,
    currchar : usize,
    errorstack : &'a mut ErrorStack
}
impl<'a> Lexer<'a> {
    pub fn new(filename : &str, errorstack : &'a mut ErrorStack) -> Lexer<'a> {
        let filename = filename.to_string();
        let s: String = fs::read_to_string(&filename).unwrap_or_else(|e| {
            eprintln!("Problem initiating lexer: {e}");
            std::process::exit(1);
        });
        Lexer { filename, tokens : Vec::new(), source : s.chars().collect(), sourcelines : s.split('\n').map(|s| s.to_string()).collect(),  curri : 0, currline : 1, currchar : 1, errorstack, }
    }
    pub fn lex(&mut self) {
        while let Some(&c) = self.source.get(self.curri) {
            if c.is_whitespace() {
                self.skip_space();
                continue;
            } 
            if c == '\\' {
                self.skip_comments();
                continue;
            }  
            if c == '"' {
                self.collect_str();
                continue;
            }
            if c.is_ascii_digit() {
                self.collect_num();
                continue;
            }
            if c.is_ascii_alphabetic() || c == '_' {
                self.collect_id();
                continue;
            }
            match c {
                '=' => { self.collect_eq(); continue; }
                '<' => { self.collect_lt(); continue; }
                '>' => { self.collect_gt(); continue; }
                '!' => { self.collect_ne(); continue; }
                _ => {  }
            }
            self.curri += 1;
            self.currchar += 1;
        }
    }
    fn skip_space(&mut self) {
        while let Some(&c) = self.source.get(self.curri) {
            if c == '\n' {
                self.currline += 1;
                self.currchar = 0;
            }
            if !c.is_whitespace() {
                return;
            }
            self.curri += 1;
            self.currchar += 1;
        }
    }
    fn skip_comments(&mut self) {
        self.curri += 1;
        self.currchar += 1;
        while let Some(&c) = self.source.get(self.curri) {
            if c == '\\' {
                self.curri += 1;
                self.currchar += 1;
                return;
            }
            self.curri += 1;
            self.currchar += 1;
        }
    }
    fn collect_str(&mut self) {
        let starting_c = self.currchar;
        self.curri += 1;
        self.currchar += 1;
        let mut s : Vec<char> = Vec::new();
        while let Some(&c) = self.source.get(self.curri) {
            if c == '"' {
                self.curri += 1;
                self.currchar += 1;
                self.tokens.push(Token::new(TokenType::STRING(s.iter().collect()), self.currline, starting_c, self.currchar));
                return;
            }
            s.push(c);
            self.curri += 1;
            self.currchar += 1;
        }
    }
    fn collect_num(&mut self) {
        let starting_c = self.currchar;
        let mut n : Vec<char> = Vec::new();
        let mut dot = false;
        while let Some(&c) = self.source.get(self.curri) {
            if c == '.' {
                dot = true;
            } 
            if !c.is_ascii_digit() && c != '.' {
                break;
            }
            n.push(c);
            self.curri += 1;
            self.currchar += 1;
        }
        let snum : String = n.iter().collect();
                if dot {
                    let num : f32 = snum.parse().unwrap_or_else( |_| {
                        self.errorstack.errors.push(
                            GError::new(ETypes::SyntaxError, "Invalid syntax for creating a float", self.filename.clone(), self.sourcelines.get(self.currline-1).unwrap().to_string(), self.currline, starting_c, self.currchar )
                        );
                        return 0.0;
                    });
                    self.tokens.push(Token::new(TokenType::FLOAT(num), self.currline, starting_c, self.currchar));
                    return;
                } else {
                    let num : i32 = snum.parse().unwrap_or_else( |_| {
                        self.errorstack.errors.push(
                            GError::new(ETypes::SyntaxError, "Invalid syntax for creating an int", self.filename.clone(), self.sourcelines.get(self.currline-1).unwrap().to_string(), self.currline, starting_c, self.currchar )
                        );
                        return 0;
                    });
                    self.tokens.push(Token::new(TokenType::INT(num), self.currline, starting_c, self.currchar));
                    return;
                }
        
    }
    fn collect_id(&mut self) {
        let starting_c = self.currchar;
        let mut id : Vec<char> = Vec::new();
        let mut first = true;
        while let Some(&c) = self.source.get(self.curri) { 
            if first {
                if !c.is_ascii_alphabetic() && c != '_' {
                    break;
                }
            } else {
                if !c.is_ascii_alphanumeric() && c != '_' {
                    break;
                }
            }
            
            id.push(c);
            self.currchar += 1;
            self.curri += 1;
            first = false;
        }
        self.tokens.push(Token::new(TokenType::ID(id.iter().collect()), self.currline, starting_c, self.currchar));
    }
    fn collect_eq(&mut self) {
        self.curri += 1;
        self.currchar += 1;
        if self.curri != self.source.len() && self.source.get(self.curri).unwrap() == &'=' {
            self.tokens.push(Token::new(TokenType::DEQL, self.currline, self.currchar-1, self.currchar));
            self.curri += 1;
            self.currchar += 1;
            return;
        }
        self.tokens.push(Token::new(TokenType::EQL, self.currline, self.currchar-1, self.currchar));
    }
    fn collect_lt(&mut self) {
        self.curri += 1;
        self.currchar += 1;
        if self.curri != self.source.len() && self.source.get(self.curri).unwrap() == &'=' {
            self.tokens.push(Token::new(TokenType::LTE, self.currline, self.currchar-1, self.currchar));
            self.curri += 1;
            self.currchar += 1;
            return;
        }
        self.tokens.push(Token::new(TokenType::LT, self.currline, self.currchar-1, self.currchar));
    }
    fn collect_gt(&mut self) {
        self.curri += 1;
        self.currchar += 1;
        if self.curri != self.source.len() && self.source.get(self.curri).unwrap() == &'=' {
            self.tokens.push(Token::new(TokenType::GTE, self.currline, self.currchar-1, self.currchar));
            self.curri += 1;
            self.currchar += 1;
            return;
        }
        self.tokens.push(Token::new(TokenType::GT, self.currline, self.currchar-1, self.currchar));
    }
    fn collect_ne(&mut self) {
        self.curri += 1;
        self.currchar += 1;
        if self.curri != self.source.len() && self.source.get(self.curri).unwrap() == &'=' {
            self.tokens.push(Token::new(TokenType::NEQ, self.currline, self.currchar-1, self.currchar));
            self.curri += 1;
            self.currchar += 1;
            return;
        }
        self.tokens.push(Token::new(TokenType::NOT, self.currline, self.currchar-1, self.currchar));
    }
    
}