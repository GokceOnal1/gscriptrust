use crate::errors::error::*;
use super::ast::*;
use super::token::*;

pub struct Parser<'a> {
    prev_token : Option<&'a Token>,
    curr_token : Option<&'a Token>,
    tokens : &'a Vec<Token>,
    errorstack : &'a mut ErrorStack,
}
impl<'a> Parser<'a> {
    pub fn new(tokens : &'a Vec<Token>, errorstack : &'a mut ErrorStack) -> Parser<'a> {
        Parser { prev_token: None, curr_token: tokens.get(0), tokens, errorstack }
    }
}