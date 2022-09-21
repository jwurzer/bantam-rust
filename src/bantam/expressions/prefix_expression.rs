
use crate::bantam::expressions::expression::Expression;
use crate::bantam::token_type::TokenType;
use std::any::Any;

/// A prefix unary arithmetic expression like "!a" or "-b".
pub struct PrefixExpression {
    operator: TokenType,
    right: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(operator: TokenType, right: Box<dyn Expression>) -> Self {
        Self{operator, right}
    }
}

impl Expression for PrefixExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        builder.push(self.operator.punctuator());
        self.right.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
