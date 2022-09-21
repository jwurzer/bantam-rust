
use crate::bantam::parselets::infix_parselet::InfixParselet;
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::precedence;
use crate::bantam::token::Token;
use crate::bantam::token_type::TokenType;
use crate::bantam::expressions::call_expression::CallExpression;
use crate::bantam::expressions::expression::Expression;

/// Parselet to parse a function call like "a(b, c, d)".
pub struct CallParselet {
}

impl CallParselet {
    pub fn new() -> Self {
        Self{}
    }
}

impl InfixParselet for CallParselet {

    fn parse(&self, parser: &mut Parser, left: Box<dyn Expression>, _token: Token) -> Result<Box<dyn Expression>, ParseError> {
        // Parse the comma-separated arguments until we hit, ")".
        let mut args: Vec<Box<dyn Expression>> = Vec::new();

        // There may be no arguments at all.
        if !parser.match_token(TokenType::RightParen) {
            loop {
                let arg: Result<Box<dyn Expression>, ParseError> = parser.parse_expression();
                if arg.is_err() {
                    return arg;
                }
                args.push(arg.unwrap());

                if !parser.match_token(TokenType::Comma) {
                    break;
                }
            }
            let consume_result = parser.consume_expected(TokenType::RightParen);
            if consume_result.is_err() {
                return Err(consume_result.err().unwrap());
            }
        }

        return Ok(Box::new(CallExpression::new(left, args)));
    }

    fn get_precedence(&self) -> i32 {
        return precedence::CALL;
    }
}
