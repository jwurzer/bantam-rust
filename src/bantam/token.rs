
use crate::bantam::token_type::TokenType;

#[derive(Clone, Eq, PartialEq)]
pub struct Token {
    token_type: TokenType,
    pub text: String, // is public to allow simple moving ownership. Used by NameParselet.
}

impl Token {
    pub fn new(token_type: TokenType, text: String) -> Self {
        Self{token_type, text}
    }
    pub fn token_type(&self) -> &TokenType { &self.token_type }
    pub fn text(&self) -> &String { &self.text }
    //pub fn text_swap(&mut self) -> String { std::mem::replace(&mut self.text, String::new()) }
    //pub fn text_move(&mut self) -> String { self.text }
}
