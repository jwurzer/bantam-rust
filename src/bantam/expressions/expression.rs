
use std::any::Any;

/// Interface for all expression AST node classes.
pub trait Expression {
    /// Pretty-print the expression to a string.
    fn print(&self, builder: &mut String) -> ();

    // Necessary to support downcasting.
    // Downcasting is used by AssignParselet to check if the left expression is a NameExpression.
    // For more infos about downcasting in Rust see:
    // https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
    // https://stackoverflow.com/questions/26126683/how-to-match-trait-implementors
    fn as_any(&self) -> &dyn Any;
}
