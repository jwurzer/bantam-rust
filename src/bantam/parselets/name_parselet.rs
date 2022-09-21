
use crate::bantam::parselets::prefix_parselet::PrefixParselet;
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::token::Token;
use crate::bantam::expressions::expression::Expression;
use crate::bantam::expressions::name_expression::NameExpression;

/// Simple parselet for a named variable like "abc".
pub struct NameParselet {
}

impl NameParselet {
    pub fn new() -> Self {
        Self{}
    }
}

impl PrefixParselet for NameParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> Result<Box<dyn Expression>, ParseError> {
        let name: String = token.text;
        return Ok(Box::new(NameExpression::new(name)));
    }
}
