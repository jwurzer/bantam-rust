
use crate::bantam::expressions::expression::Expression;
use std::any::Any;

/// A ternary conditional expression like "a ? b : c".
pub struct ConditionalExpression {
    condition: Box<dyn Expression>,
    then_arm: Box<dyn Expression>,
    else_arm: Box<dyn Expression>,
}

impl ConditionalExpression {
    pub fn new(condition: Box<dyn Expression>,
           then_arm: Box<dyn Expression>,
           else_arm: Box<dyn Expression>) -> Self {
        Self{condition, then_arm, else_arm}
    }
}

impl Expression for ConditionalExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        self.condition.print(builder);
        builder.push_str(" ? ");
        self.then_arm.print(builder);
        builder.push_str(" : ");
        self.else_arm.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
