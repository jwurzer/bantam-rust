
use crate::bantam::expressions::expression::Expression;
use crate::bantam::token_type::TokenType;
use std::any::Any;

/// A binary arithmetic expression like "a + b" or "c ^ d".
pub struct OperatorExpression {
    left: Box<dyn Expression>,
    operator: TokenType,
    right: Box<dyn Expression>,
}

impl OperatorExpression {
    pub fn new(left: Box<dyn Expression>, operator: TokenType, right: Box<dyn Expression>) -> Self {
        Self{left, operator, right}
    }
}

impl Expression for OperatorExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        self.left.print(builder);
        builder.push_str(" ");
        builder.push(self.operator.punctuator());
        builder.push_str(" ");
        self.right.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
