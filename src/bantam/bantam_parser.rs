use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::precedence;
use crate::bantam::token::Token;
use crate::bantam::token_type::TokenType;
use crate::bantam::parselets::prefix_parselet::PrefixParselet;
use crate::bantam::parselets::infix_parselet::InfixParselet;
use crate::bantam::parselets::assign_parselet::AssignParselet;
use crate::bantam::parselets::binary_operator_parselet::BinaryOperatorParselet;
use crate::bantam::parselets::call_parselet::CallParselet;
use crate::bantam::parselets::conditional_parselet::ConditionalParselet;
use crate::bantam::parselets::group_parselet::GroupParselet;
use crate::bantam::parselets::name_parselet::NameParselet;
use crate::bantam::parselets::postfix_operator_parselet::PostfixOperatorParselet;
use crate::bantam::parselets::prefix_operator_parselet::PrefixOperatorParselet;
use crate::bantam::expressions::expression::Expression;

pub struct BantamParser {
    parser: Parser,
}

impl BantamParser {
    pub fn new(tokens: Box<dyn Iterator<Item=Token>>) -> Self {
        let mut bp: BantamParser = Self { parser: Parser::new(tokens) };

        // Register all of the parselets for the grammar.

        // Register the ones that need special parselets.
        bp.register_prefix(TokenType::Name,       Box::new(NameParselet::new()));
        bp.register_infix(TokenType::Assign,      Box::new(AssignParselet::new()));
        bp.register_infix(TokenType::Question,    Box::new(ConditionalParselet::new()));
        bp.register_prefix(TokenType::LeftParen,  Box::new(GroupParselet::new()));
        bp.register_infix(TokenType::LeftParen,  Box::new(CallParselet::new()));

        // Register the simple operator parselets.
        bp.prefix(TokenType::Plus,      precedence::PREFIX);
        bp.prefix(TokenType::Minus,     precedence::PREFIX);
        bp.prefix(TokenType::Tilde,     precedence::PREFIX);
        bp.prefix(TokenType::Bang,      precedence::PREFIX);

        // For kicks, we'll make "!" both prefix and postfix, kind of like ++.
        bp.postfix(TokenType::Bang,     precedence::POSTFIX);

        bp.infix_left(TokenType::Plus,     precedence::SUM);
        bp.infix_left(TokenType::Minus,    precedence::SUM);
        bp.infix_left(TokenType::Asterisk, precedence::PRODUCT);
        bp.infix_left(TokenType::Slash,    precedence::PRODUCT);
        bp.infix_right(TokenType::Caret,   precedence::EXPONENT);

        return bp;
    }

    pub fn register_prefix(&mut self, token: TokenType, parselet: Box<dyn PrefixParselet>) -> () {
        self.parser.register_prefix(token, parselet);
    }

    pub fn register_infix(&mut self, token: TokenType, parselet: Box<dyn InfixParselet>) -> () {
        self.parser.register_infix(token, parselet);
    }

    /// Registers a postfix unary operator parselet for the given token and
    /// precedence.
    pub fn postfix(&mut self, token: TokenType, precedence: i32) -> () {
        self.register_infix(token, Box::new(PostfixOperatorParselet::new(precedence)));
    }

    /// Registers a prefix unary operator parselet for the given token and
    /// precedence.
    pub fn prefix(&mut self, token: TokenType, precedence: i32) -> () {
        self.register_prefix(token, Box::new(PrefixOperatorParselet::new(precedence)));
    }

    /// Registers a left-associative binary operator parselet for the given token
    /// and precedence.
    pub fn infix_left(&mut self, token: TokenType, precedence: i32) -> () {
        self.register_infix(token, Box::new(BinaryOperatorParselet::new(precedence, false)));
    }

    /// Registers a right-associative binary operator parselet for the given token
    /// and precedence.
    pub fn infix_right(&mut self, token: TokenType, precedence: i32) -> () {
        self.register_infix(token, Box::new(BinaryOperatorParselet::new(precedence, true)));
    }

    pub fn parse_expression(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        return self.parser.parse_expression();
    }
}
