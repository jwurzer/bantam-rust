
use crate::bantam::parselets::infix_parselet::InfixParselet;
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

    fn parse(&self, parser: &mut Parser, left: Box<dyn Expression>, _token: Token) -> Box<dyn Expression> {
        let then_arm: Box<dyn Expression> = parser.parse_expression();
        parser.consume_expected(TokenType::Colon);
        let else_arm: Box<dyn Expression> = parser.parse_expression_precedence(precedence::CONDITIONAL - 1);

        return Box::new(ConditionalExpression::new(left, then_arm, else_arm));
    }

    fn get_precedence(&self) -> i32 {
        return precedence::CONDITIONAL;
    }
}
