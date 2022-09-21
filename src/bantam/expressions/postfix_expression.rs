
use crate::bantam::expressions::expression::Expression;
use crate::bantam::token_type::TokenType;
use std::any::Any;

/// A postfix unary arithmetic expression like "a!".
pub struct PostfixExpression {
    left: Box<dyn Expression>,
    operator: TokenType,
}

impl PostfixExpression {
    pub fn new(left: Box<dyn Expression>, operator: TokenType) -> Self {
        Self{left, operator}
    }
}

impl Expression for PostfixExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        self.left.print(builder);
        builder.push(self.operator.punctuator());
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
