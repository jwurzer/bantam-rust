
use crate::bantam::parselets::prefix_parselet::PrefixParselet;
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::token::Token;
use crate::bantam::token_type::TokenType;
use crate::bantam::expressions::expression::Expression;

/// Parses parentheses used to group an expression, like "a * (b + c)".
pub struct GroupParselet {
}

impl GroupParselet {
    pub fn new() -> Self {
        Self{}
    }
}

impl PrefixParselet for GroupParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> Result<Box<dyn Expression>, ParseError> {
        //let expression: Box<dyn Expression> = parser.parse_expression();
        let expression = parser.parse_expression();
        let consume_result = parser.consume_expected(TokenType::RightParen);
        if consume_result.is_err() {
            return Err(consume_result.err().unwrap());
        }
        return expression;
    }
}
