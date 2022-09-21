
use crate::bantam::parselets::infix_parselet::InfixParselet;
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::token::Token;
use crate::bantam::expressions::expression::Expression;
use crate::bantam::expressions::operator_expression::OperatorExpression;

/// Generic infix parselet for a binary arithmetic operator. The only
/// difference when parsing, "+", "-", "*", "/", and "^" is precedence and
/// associativity, so we can use a single parselet class for all of those.
pub struct BinaryOperatorParselet {
    precedence: i32,
    is_right: bool,
}

impl BinaryOperatorParselet {
    pub fn new(precedence: i32, is_right: bool) -> Self {
        Self{precedence, is_right}
    }
}

impl InfixParselet for BinaryOperatorParselet {

    fn parse(&self, parser: &mut Parser, left: Box<dyn Expression>, token: Token) -> Result<Box<dyn Expression>, ParseError> {
        // To handle right-associative operators like "^", we allow a slightly
        // lower precedence when parsing the right-hand side. This will let a
        // parselet with the same precedence appear on the right, which will then
        // take *this* parselet's result as its left-hand argument.
        let right: Result<Box<dyn Expression>, ParseError> = parser.parse_expression_precedence(
            self.precedence - if self.is_right { 1 } else { 0 });
        if right.is_err() {
            return right;
        }
        return Ok(Box::new(OperatorExpression::new(left, *token.token_type(), right.unwrap())));
    }

    fn get_precedence(&self) -> i32 {
        return self.precedence;
    }
}
