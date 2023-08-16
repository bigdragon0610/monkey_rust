pub(crate) const ILLEGAL: TokenType = "ILLEGAL";
pub(crate) const EOF: TokenType = "EOF";

pub(crate) const IDENT: TokenType = "IDENT";
pub(crate) const INT: TokenType = "INT";

pub(crate) const ASSIGN: TokenType = "=";
pub(crate) const PLUS: TokenType = "+";
pub(crate) const MINUS: TokenType = "-";
pub(crate) const BANG: TokenType = "!";
pub(crate) const ASTERISK: TokenType = "*";
pub(crate) const SLASH: TokenType = "/";

pub(crate) const LT: TokenType = "<";
pub(crate) const GT: TokenType = ">";

pub(crate) const COMMA: TokenType = ",";
pub(crate) const SEMICOLON: TokenType = ";";

pub(crate) const LPAREN: TokenType = "(";
pub(crate) const RPAREN: TokenType = ")";
pub(crate) const LBRACE: TokenType = "{";
pub(crate) const RBRACE: TokenType = "}";

pub(crate) const FUNCTION: TokenType = "FUNCTION";
pub(crate) const LET: TokenType = "LET";
pub(crate) const TRUE: TokenType = "TRUE";
pub(crate) const FALSE: TokenType = "FALSE";
pub(crate) const IF: TokenType = "IF";
pub(crate) const ELSE: TokenType = "ELSE";
pub(crate) const RETURN: TokenType = "RETURN";

pub(crate) const EQ: TokenType = "==";
pub(crate) const NOT_EQ: TokenType = "!=";

const KEYWORDS: [(&str, TokenType); 7] = [
    ("fn", FUNCTION),
    ("let", LET),
    ("true", TRUE),
    ("false", FALSE),
    ("if", IF),
    ("else", ELSE),
    ("return", RETURN),
];

pub(crate) fn lookup_ident(ident: &str) -> TokenType {
    if let Some(tok) = KEYWORDS
        .iter()
        .find_map(|kw| if kw.0 == ident { Some(kw.1) } else { None })
    {
        return tok;
    }
    IDENT
}

pub(crate) type TokenType = &'static str;

pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}
