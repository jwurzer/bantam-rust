
// The two interfaces PrefixParselet and InfixParselet (traits) used by the Pratt parser.
pub mod prefix_parselet;
pub mod infix_parselet;

// Implementations for PrefixParselet
pub mod name_parselet;
pub mod group_parselet;
pub mod prefix_operator_parselet;

// Implementations for InfixParselet
pub mod assign_parselet;
pub mod binary_operator_parselet;
pub mod call_parselet;
pub mod conditional_parselet;
pub mod postfix_operator_parselet;
