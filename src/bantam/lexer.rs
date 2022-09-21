use crate::bantam::token_type::TokenType;
use crate::bantam::token::Token;
use std::collections::HashMap;

/// A very primitive lexer. Takes a string and splits it into a series of
/// Tokens. Operators and punctuation are mapped to unique keywords. Names,
/// which can be any series of letters, are turned into NAME tokens. All other
/// characters are ignored (except to separate names). Numbers and strings are
/// not supported. This is really just the bare minimum to give the parser
/// something to work with.
pub struct Lexer {
    punctuators: HashMap<char, TokenType>,
    text: Vec<char>, // Using Vec<char> instead of String for simpler iteration.
    index: usize,
}

impl Lexer {
    /// Creates a new Lexer to tokenize the given string.
    ///
    /// # Arguments
    ///
    /// * `text` - String to tokenize.
    pub fn new(text: String) -> Self {
        let mut lexer = Self{punctuators: HashMap::new(), text: text.chars().collect(), index: 0};
        // Register all of the TokenTypes that are explicit punctuators.
        for token_type in TokenType::values() {
            let punctuator: char = token_type.punctuator();
            if punctuator != '\0' {
                lexer.punctuators.insert(punctuator, *token_type);
			}
        }
        lexer
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.text.len() {
            let c: char = self.text[self.index];
            self.index += 1;
            if self.punctuators.contains_key(&c) {
                // Handle punctuation.
                return Some(Token::new(*self.punctuators.get(&c).unwrap(), String::from(c)));
            } else if c.is_alphabetic() {
                // Handle names.
				let start: usize = self.index - 1;
				while self.index < self.text.len() {
                    let c: char = self.text[self.index];
					if !c.is_alphabetic() { break; }
					self.index += 1;
				}
                let name: String = self.text[start..self.index].iter().collect();
                return Some(Token::new(TokenType::Name, name));
            } else {
                // Ignore all other characters (whitespace, etc.)
            }
        }

        // Once we've reached the end of the string, just return EOF tokens. We'll
        // just keeping returning them as many times as we're asked so that the
        // parser's lookahead doesn't have to worry about running out of tokens.
        return Some(Token::new(TokenType::Eof, String::new()));
    }
}
