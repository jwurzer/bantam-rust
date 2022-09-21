
use crate::bantam::expressions::expression::Expression;
use std::any::Any;

/// A simple variable name expression like "abc".
pub struct NameExpression {
    name: String,
}

impl NameExpression {
    pub fn new(name: String) -> Self {
        Self{name}
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Expression for NameExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str(&self.name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
