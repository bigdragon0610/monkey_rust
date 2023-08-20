use std::{iter::Peekable, str::Chars};

use crate::token::{
    lookup_ident, Token, TokenType, ASSIGN, ASTERISK, BANG, COMMA, EOF, EQ, GT, ILLEGAL, INT,
    LBRACE, LPAREN, LT, MINUS, NOT_EQ, PLUS, RBRACE, RPAREN, SEMICOLON, SLASH,
};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.read_char()
            .map(|ch| match ch {
                '=' => {
                    if let Some(tok) =
                        self.peek_char()
                            .filter(|&peek_ch| peek_ch == '=')
                            .map(|peek_ch| Token {
                                token_type: EQ,
                                literal: format!("{}{}", ch, peek_ch),
                            })
                    {
                        self.read_char();
                        tok
                    } else {
                        new_token(ASSIGN, ch)
                    }
                }
                '+' => new_token(PLUS, ch),
                '-' => new_token(MINUS, ch),
                '!' => {
                    if let Some(tok) =
                        self.peek_char()
                            .filter(|&peek_ch| peek_ch == '=')
                            .map(|peek_ch| Token {
                                token_type: NOT_EQ,
                                literal: format!("{}{}", ch, peek_ch),
                            })
                    {
                        self.read_char();
                        tok
                    } else {
                        new_token(BANG, ch)
                    }
                }
                '/' => new_token(SLASH, ch),
                '*' => new_token(ASTERISK, ch),
                '<' => new_token(LT, ch),
                '>' => new_token(GT, ch),
                ';' => new_token(SEMICOLON, ch),
                ',' => new_token(COMMA, ch),
                '(' => new_token(LPAREN, ch),
                ')' => new_token(RPAREN, ch),
                '{' => new_token(LBRACE, ch),
                '}' => new_token(RBRACE, ch),
                _ => {
                    if is_letter(ch) {
                        let literal = self.read_identifier(ch);
                        Token {
                            token_type: lookup_ident(&literal),
                            literal,
                        }
                    } else if ch.is_ascii_digit() {
                        Token {
                            token_type: INT,
                            literal: self.read_number(ch),
                        }
                    } else {
                        new_token(ILLEGAL, ch)
                    }
                }
            })
            .unwrap_or(new_token(EOF, ' '))
    }

    fn read_identifier(&mut self, ch: char) -> String {
        let mut ident = ch.to_string();
        while let Some(ch) = self.peek_char() {
            if !is_letter(ch) {
                break;
            }
            ident = format!("{}{}", ident, ch);
            self.read_char();
        }
        ident
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if !ch.is_ascii_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self, ch: char) -> String {
        let mut int = ch.to_string();
        while let Some(ch) = self.peek_char() {
            if !ch.is_ascii_digit() {
                break;
            }
            int = format!("{}{}", int, ch);
            self.read_char();
        }
        int
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().copied()
    }
}

fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{
        ASSIGN, ASTERISK, BANG, COMMA, ELSE, EOF, EQ, FALSE, FUNCTION, GT, IDENT, IF, INT, LBRACE,
        LET, LPAREN, LT, MINUS, NOT_EQ, PLUS, RBRACE, RETURN, RPAREN, SEMICOLON, SLASH, TRUE,
    };

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
          return true;
        } else {
          return false;
        }

        10 == 10;
        10 != 9;
        "
        .to_string();

        let tests = [
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INT, "5"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "result"),
            (ASSIGN, "="),
            (IDENT, "add"),
            (LPAREN, "("),
            (IDENT, "five"),
            (COMMA, ","),
            (IDENT, "ten"),
            (RPAREN, ")"),
            (SEMICOLON, ";"),
            (BANG, "!"),
            (MINUS, "-"),
            (SLASH, "/"),
            (ASTERISK, "*"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (GT, ">"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (IF, "if"),
            (LPAREN, "("),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (INT, "10"),
            (EQ, "=="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (INT, "10"),
            (NOT_EQ, "!="),
            (INT, "9"),
            (SEMICOLON, ";"),
            (EOF, " "),
        ];

        let mut l = Lexer::new(&input);

        for (i, tt) in tests.into_iter().enumerate() {
            let tok = l.next_token();
            if tok.token_type != tt.0 {
                panic!(
                    "tests[{}] - tokentype wrong. expected={}, got={}",
                    i, tt.0, tok.token_type
                );
            }

            if tok.literal != tt.1 {
                panic!(
                    "tests[{}] - Literal wrong. expected={}, got={}",
                    i, tt.1, tok.literal
                )
            }
        }
    }
}
