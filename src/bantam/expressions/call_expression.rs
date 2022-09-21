
use crate::bantam::expressions::expression::Expression;
use std::any::Any;

/// A function call like "a(b, c, d)".
pub struct CallExpression {
    function: Box<dyn Expression>,
    args: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(function: Box<dyn Expression>, args: Vec<Box<dyn Expression>>) -> Self {
        Self{function, args}
    }
}

impl Expression for CallExpression {
    fn print(&self, builder: &mut String) -> () {
        self.function.print(builder);
        builder.push_str("(");
        let mut i: usize = 0;
        for arg in self.args.iter() {
            arg.print(builder);
            if i + 1 < self.args.len() {
                builder.push_str(", ");
            }
            i += 1;
        }
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
