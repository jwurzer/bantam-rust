
use crate::bantam::parselets::infix_parselet::InfixParselet;
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::precedence;
use crate::bantam::token::Token;
use crate::bantam::expressions::assign_expression::AssignExpression;
use crate::bantam::expressions::expression::Expression;
use crate::bantam::expressions::name_expression::NameExpression;

/// Parses assignment expressions like "a = b". The left side of an assignment
/// expression must be a simple name like "a", and expressions are
/// right-associative. (In other words, "a = b = c" is parsed as "a = (b = c)").
pub struct AssignParselet {
}

impl AssignParselet {
    pub fn new() -> Self {
        Self{}
    }
}

impl InfixParselet for AssignParselet {

    fn parse(&self, parser: &mut Parser, left: Box<dyn Expression>, _token: Token) -> Result<Box<dyn Expression>, ParseError> {
        let right: Result<Box<dyn Expression>, ParseError> = parser.parse_expression_precedence(precedence::ASSIGNMENT - 1);
        if right.is_err() {
            return right;
        }

        let left_name_expr: &NameExpression = match left.as_any().downcast_ref::<NameExpression>() {
            Some(ne) => ne,
            None => return Err(ParseError::new("The left-hand side of an assignment must be a name.".to_string()))
        };

        let name: &String = left_name_expr.name();
        return Ok(Box::new(AssignExpression::new(name.clone(), right.unwrap())));
    }

    fn get_precedence(&self) -> i32 {
        return precedence::ASSIGNMENT;
    }
}
