#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum TokenType {
    #[default]
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NOTEQ,
}

const KEYWORDS: [(&str, TokenType); 7] = [
    ("fn", TokenType::FUNCTION),
    ("let", TokenType::LET),
    ("true", TokenType::TRUE),
    ("false", TokenType::FALSE),
    ("if", TokenType::IF),
    ("else", TokenType::ELSE),
    ("return", TokenType::RETURN),
];

pub fn lookup_ident(ident: &str) -> TokenType {
    if let Some(tok) = KEYWORDS
        .iter()
        .find_map(|kw| if kw.0 == ident { Some(kw.1) } else { None })
    {
        return tok;
    }
    TokenType::IDENT
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new() -> Self {
        Token {
            token_type: TokenType::default(),
            literal: String::default(),
        }
    }
}
