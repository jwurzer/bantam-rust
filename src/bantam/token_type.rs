
// If the enum TokenType is changed or extended with some new values then
// don't forget to update the values() function (see below).
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Comma,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Tilde,
    Bang,
    Question,
    Colon,
    Name,
    Eof,
}

impl TokenType {
    pub fn values() -> &'static [TokenType; 15] {
        // Extra array to provide all token types. Is needed by lexer.
        const TOKEN_TYPES: [TokenType; 15] = [
            TokenType::LeftParen, TokenType::RightParen, TokenType::Comma, TokenType::Assign,
            TokenType::Plus, TokenType::Minus, TokenType::Asterisk, TokenType::Slash,
            TokenType::Caret, TokenType::Tilde, TokenType::Bang, TokenType::Question,
            TokenType::Colon, TokenType::Name, TokenType::Eof];

        return &TOKEN_TYPES;
    }

    pub fn punctuator(&self) -> char {
        match *self {
            TokenType::LeftParen => '(',
            TokenType::RightParen => ')',
            TokenType::Comma => ',',
            TokenType::Assign => '=',
            TokenType::Plus => '+',
            TokenType::Minus => '-',
            TokenType::Asterisk => '*',
            TokenType::Slash => '/',
            TokenType::Caret => '^',
            TokenType::Tilde => '~',
            TokenType::Bang => '!',
            TokenType::Question => '?',
            TokenType::Colon => ':',
            _ => '\0',
        }
    }

    pub fn to_string(&self) -> &'static str {
        match *self {
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::Comma => "Comma",
            TokenType::Assign => "Assign",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Asterisk => "Asterisk",
            TokenType::Slash => "Slash",
            TokenType::Caret => "Caret",
            TokenType::Tilde => "Tilde",
            TokenType::Bang => "Bang",
            TokenType::Question => "Question",
            TokenType::Colon => "Colon",
            TokenType::Name => "Name",
            TokenType::Eof => "Eof",
        }
    }
}
