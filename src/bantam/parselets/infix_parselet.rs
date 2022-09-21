
use crate::bantam::parse_error::ParseError;
use crate::bantam::parser::Parser;
use crate::bantam::token::Token;
use crate::bantam::expressions::expression::Expression;

/// One of the two parselet interfaces used by the Pratt parser. An
/// InfixParselet is associated with a token that appears in the middle of the
/// expression it parses. Its parse() method will be called after the left-hand
/// side has been parsed, and it in turn is responsible for parsing everything
/// that comes after the token. This is also used for postfix expressions, in
/// which case it simply doesn't consume any more tokens in its parse() call.
pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<dyn Expression>, token: Token) -> Result<Box<dyn Expression>, ParseError>;
    fn get_precedence(&self) -> i32;
}
