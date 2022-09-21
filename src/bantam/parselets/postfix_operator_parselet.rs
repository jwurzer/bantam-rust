
use crate::bantam::parselets::infix_parselet::InfixParselet;
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::token::Token;
use crate::bantam::expressions::expression::Expression;
use crate::bantam::expressions::postfix_expression::PostfixExpression;

/// Generic infix parselet for an unary arithmetic operator. Parses postfix
/// unary "?" expressions.
pub struct PostfixOperatorParselet {
    precedence: i32,
}

impl PostfixOperatorParselet {
    pub fn new(precedence: i32) -> Self {
        Self{precedence}
    }
}

impl InfixParselet for PostfixOperatorParselet {

    fn parse(&self, _parser: &mut Parser, left: Box<dyn Expression>, token: Token) -> Result<Box<dyn Expression>, ParseError> {
        return Ok(Box::new(PostfixExpression::new(left, *token.token_type())));
    }

    fn get_precedence(&self) -> i32 {
        return self.precedence;
    }
}
