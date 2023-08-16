pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";

pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";

pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";

const KEYWORDS: [(&str, TokenType); 2] = [("fn", FUNCTION), ("let", LET)];

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
