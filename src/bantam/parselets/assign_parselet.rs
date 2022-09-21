
use crate::bantam::parselets::infix_parselet::InfixParselet;
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

    fn parse(&self, parser: &mut Parser, left: Box<dyn Expression>, _token: Token) -> Box<dyn Expression> {
        let right: Box<dyn Expression> = parser.parse_expression_precedence(precedence::ASSIGNMENT - 1);

        let left_name_expr: &NameExpression = match left.as_any().downcast_ref::<NameExpression>() {
            Some(ne) => ne,
            None => panic!("The left-hand side of an assignment must be a name."),
        };

        let name: &String = left_name_expr.name();
        return Box::new(AssignExpression::new(name.clone(), right));
    }

    fn get_precedence(&self) -> i32 {
        return precedence::ASSIGNMENT;
    }
}
