use super::token::Token;
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
        Lexer { filename, tokens : Vec::new(), source : s.chars().collect(), sourcelines : s.split('\n').map(|s| s.to_string()).collect(),  curri : 0, currline : 1, currchar : 0, errorstack, }
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
            //println!("CHAR: {} - INDEX: {}", c.to_string().red(), self.curri.to_string().blue());
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
        self.curri += 1;
        self.currchar += 1;
        let mut s : Vec<char> = Vec::new();
        while let Some(&c) = self.source.get(self.curri) {
            if c == '"' {
                self.curri += 1;
                self.currchar += 1;
                self.tokens.push(Token::STRING(s.iter().collect()));
                return;
            }
            s.push(c);
            self.curri += 1;
            self.currchar += 1;
        }
    }
    fn collect_num(&mut self) {
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
                    let num : f32 = snum.parse().unwrap_or_else( |e| {
                        self.errorstack.errors.push(
                            GError::new(ETypes::SyntaxError, "Invalid syntax for creating a float", self.filename.clone(), self.sourcelines.get(self.currline-1).unwrap().to_string(), self.currline, self.currchar )
                        );
                        return 0.0;
                    });
                    self.tokens.push(Token::FLOAT(num));
                    return;
                } else {
                    let num : i32 = snum.parse().unwrap_or_else( |e| {
                        println!("Invalid syntax for creating int: {e}");
                        return 0;
                    });
                    self.tokens.push(Token::INT(num));
                    return;
                }
        
    }
    
}