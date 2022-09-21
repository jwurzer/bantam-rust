
use crate::bantam::expressions::expression::Expression;
use std::any::Any;

/// An assignment expression like "a = b".
pub struct AssignExpression {
    name: String,
    right: Box<dyn Expression>,
}

impl AssignExpression {
    pub fn new(name: String, right: Box<dyn Expression>) -> Self {
        Self{name, right}
    }
}

impl Expression for AssignExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        builder.push_str(&self.name);
        builder.push_str(" = ");
        self.right.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
