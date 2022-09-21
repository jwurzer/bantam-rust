
/**
 * Defines the different precedence levels used by the infix parsers. These
 * determine how a series of infix expressions will be grouped. For example,
 * "a + b * c - d" will be parsed as "(a + (b * c)) - d" because "*" has higher
 * precedence than "+" and "-". Here, bigger numbers mean higher precedence.
 */
pub const ASSIGNMENT: i32  = 1;
pub const CONDITIONAL: i32 = 2;
pub const SUM: i32         = 3;
pub const PRODUCT: i32     = 4;
pub const EXPONENT: i32    = 5;
pub const PREFIX: i32      = 6;
pub const POSTFIX: i32     = 7;
pub const CALL: i32        = 8;
