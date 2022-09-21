
use crate::bantam::parselets::infix_parselet::InfixParselet;
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::precedence;
use crate::bantam::token::Token;
use crate::bantam::token_type::TokenType;
use crate::bantam::expressions::conditional_expression::ConditionalExpression;
use crate::bantam::expressions::expression::Expression;

// Parselet for the condition or "ternary" operator, like "a ? b : c".
pub struct ConditionalParselet {
}

impl ConditionalParselet {
    pub fn new() -> Self {
        Self{}
    }
}

impl InfixParselet for ConditionalParselet {

    fn parse(&self, parser: &mut Parser, left: Box<dyn Expression>, _token: Token) -> Result<Box<dyn Expression>, ParseError> {
        let then_arm: Result<Box<dyn Expression>, ParseError> = parser.parse_expression();
        if then_arm.is_err() {
            return then_arm;
        }
        let consume_result = parser.consume_expected(TokenType::Colon);
        if consume_result.is_err() {
            return Err(consume_result.err().unwrap());
        }
        let else_arm: Result<Box<dyn Expression>, ParseError> = parser.parse_expression_precedence(precedence::CONDITIONAL - 1);
        if else_arm.is_err() {
            return else_arm;
        }

        return Ok(Box::new(ConditionalExpression::new(left, then_arm.unwrap(), else_arm.unwrap())));
    }

    fn get_precedence(&self) -> i32 {
        return precedence::CONDITIONAL;
    }
}
