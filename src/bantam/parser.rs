use crate::bantam::token_type::TokenType;
use crate::bantam::token::Token;
use crate::bantam::expressions::expression::Expression;
use crate::bantam::parselets::infix_parselet::InfixParselet;
use crate::bantam::parselets::prefix_parselet::PrefixParselet;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Parser {
    tokens: Box<dyn Iterator<Item = Token>>,
    read: Vec<Token>,
    prefix_parselets: HashMap<TokenType, Rc<dyn PrefixParselet>>,
    infix_parselets: HashMap<TokenType, Rc<dyn InfixParselet>>,
}

impl Parser {
    pub fn new(tokens: Box<dyn Iterator<Item = Token>>) -> Self {
        Self{tokens, read: Vec::new(), prefix_parselets: HashMap::new(), infix_parselets: HashMap::new()}
    }

    pub fn register_prefix(&mut self, token: TokenType, parselet: Box<dyn PrefixParselet>) -> () {
        self.prefix_parselets.insert(token, Rc::from(parselet));
    }

    pub fn register_infix(&mut self, token: TokenType, parselet: Box<dyn InfixParselet>) -> () {
        self.infix_parselets.insert(token, Rc::from(parselet));
    }

    pub fn parse_expression_precedence(&mut self, precedence: i32) -> Box<dyn Expression> {
        let mut token: Token = self.consume();

        // Using Rc instead of Box. Otherwise: immutable borrow occurs here
        let prefix_opt: Option<&Rc<dyn PrefixParselet>> = self.prefix_parselets.get(token.token_type());
        if prefix_opt.is_none() {
            panic!("Could not parse \"{}\".", token.text());
        }
        let prefix: Rc<dyn PrefixParselet> = prefix_opt.unwrap().clone();

        let mut left: Box<dyn Expression> = prefix.parse(self, token);

        while precedence < self.get_precedence() {
            token = self.consume();

            let infix_opt: Option<&Rc<dyn InfixParselet>> = self.infix_parselets.get(token.token_type());
            let infix: Rc<dyn InfixParselet> = infix_opt.unwrap().clone();
            left = infix.parse(self, left, token);
        }

        return left;
    }

    pub fn parse_expression(&mut self) -> Box<dyn Expression> {
        return self.parse_expression_precedence(0);
    }

    pub fn match_token(&mut self, expected: TokenType) -> bool {
        let token: Token = self.look_ahead(0);
        if *token.token_type() != expected {
            return false;
        }

        self.consume();
        return true;
    }

    pub fn consume_expected(&mut self, expected: TokenType) -> Token {
        let token: Token = self.look_ahead(0);
        if *token.token_type() != expected {
            panic!("Expected token {} and found {}", expected.to_string(), token.token_type().to_string());
        }

        return self.consume();
    }

    pub fn consume(&mut self) -> Token {
        // Make sure we've read the token.
		self.look_ahead(0);

        return self.read.remove(0);
    }

    fn look_ahead(&mut self, distance: usize) -> Token {
        // Read in as many as needed.
		while distance >= self.read.len() {
            self.read.push(self.tokens.next().unwrap());
		}

        // Get the queued token.
		return self.read[distance].clone();
    }

    fn get_precedence(&mut self) -> i32 {
        let token_type: TokenType = *self.look_ahead(0).token_type();
        let parser: Option<&Rc<dyn InfixParselet>> = self.infix_parselets.get(&token_type);
        if parser.is_some() {
            return parser.unwrap().get_precedence();
        }
        return 0;
    }
}