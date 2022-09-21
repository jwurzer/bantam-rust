
use crate::bantam::parselets::prefix_parselet::PrefixParselet;
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
    fn parse(&self, parser: &mut Parser, _token: Token) -> Box<dyn Expression> {
        let expression: Box<dyn Expression> = parser.parse_expression();
        parser.consume_expected(TokenType::RightParen);
        return expression;
    }
}
