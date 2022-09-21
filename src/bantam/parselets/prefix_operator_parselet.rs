
use crate::bantam::parselets::prefix_parselet::PrefixParselet;
use crate::bantam::parser::Parser;
use crate::bantam::token::Token;
use crate::bantam::expressions::expression::Expression;
use crate::bantam::expressions::prefix_expression::PrefixExpression;

/// Generic prefix parselet for an unary arithmetic operator. Parses prefix
/// unary "-", "+", "~", and "!" expressions.
pub struct PrefixOperatorParselet {
    precedence: i32,
}

impl PrefixOperatorParselet {
    pub fn new(precedence: i32) -> Self {
        Self{precedence}
    }

    // not necessary. The trait PrefixParselet doesn't have get_precedence().
    //pub fn get_precedence(&self) -> i32 {
    //    return self.precedence;
    //}
}

impl PrefixParselet for PrefixOperatorParselet {

    fn parse(&self, parser: &mut Parser, token: Token) -> Box<dyn Expression> {
        // To handle right-associative operators like "^", we allow a slightly
        // lower precedence when parsing the right-hand side. This will let a
        // parselet with the same precedence appear on the right, which will then
        // take *this* parselet's result as its left-hand argument.
        let right: Box<dyn Expression> = parser.parse_expression_precedence(self.precedence);
        return Box::new(PrefixExpression::new(*token.token_type(), right));
    }
}
