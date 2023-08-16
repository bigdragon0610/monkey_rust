pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";

pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";

pub const LT: TokenType = "<";
pub const GT: TokenType = ">";

pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "IF";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";

pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";

const KEYWORDS: [(&str, TokenType); 7] = [
    ("fn", FUNCTION),
    ("let", LET),
    ("true", TRUE),
    ("false", FALSE),
    ("if", IF),
    ("else", ELSE),
    ("return", RETURN),
];

pub fn lookup_ident(ident: &str) -> TokenType {
    if let Some(tok) = KEYWORDS
        .iter()
        .find_map(|kw| if kw.0 == ident { Some(kw.1) } else { None })
    {
        return tok;
    }
    IDENT
}

pub type TokenType = &'static str;

pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}
