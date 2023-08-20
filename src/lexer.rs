use crate::token::{
    lookup_ident, Token,
    TokenType::{
        self, ASSIGN, ASTERISK, BANG, COMMA, EOF, EQ, GT, ILLEGAL, INT, LBRACE, LPAREN, LT, MINUS,
        NOTEQ, PLUS, RBRACE, RPAREN, SEMICOLON, SLASH,
    },
};

#[derive(Debug)]
pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input }
    }

    pub fn next_token(&self, position: usize) -> (Token, usize) {
        let mut position = self.skip_whitespace(position);
        let tok = self
            .read_char(position)
            .map(|ch| match ch {
                '=' => {
                    if let Some(tok) = self
                        .read_char(position + 1)
                        .filter(|&peek_ch| peek_ch == '=')
                        .map(|peek_ch| Token {
                            token_type: EQ,
                            literal: format!("{}{}", ch, peek_ch),
                        })
                    {
                        position += 1;
                        tok
                    } else {
                        new_token(ASSIGN, ch)
                    }
                }
                '+' => new_token(PLUS, ch),
                '-' => new_token(MINUS, ch),
                '!' => {
                    if let Some(tok) = self
                        .read_char(position + 1)
                        .filter(|&peek_ch| peek_ch == '=')
                        .map(|peek_ch| Token {
                            token_type: NOTEQ,
                            literal: format!("{}{}", ch, peek_ch),
                        })
                    {
                        position += 1;
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
                        let read_position = self.read_identifier(position);
                        let literal = self.input[position..read_position].to_string();
                        position = read_position - 1;
                        Token {
                            token_type: lookup_ident(&literal),
                            literal,
                        }
                    } else if ch.is_ascii_digit() {
                        let read_position = self.read_number(position);
                        let literal = self.input[position..read_position].to_string();
                        position = read_position - 1;
                        Token {
                            token_type: INT,
                            literal,
                        }
                    } else {
                        new_token(ILLEGAL, ch)
                    }
                }
            })
            .unwrap_or(new_token(EOF, ' '));
        (tok, position + 1)
    }

    fn read_char(&self, position: usize) -> Option<char> {
        self.input.chars().nth(position)
    }

    fn read_identifier(&self, position: usize) -> usize {
        let mut position = position;
        while let Some(ch) = self.read_char(position) {
            if !is_letter(ch) {
                break;
            }
            position += 1;
        }
        position
    }

    fn skip_whitespace(&self, position: usize) -> usize {
        let mut position = position;
        while let Some(ch) = self.read_char(position) {
            if !ch.is_ascii_whitespace() {
                break;
            }
            position += 1;
        }
        position
    }

    fn read_number(&self, position: usize) -> usize {
        let mut position = position;
        while let Some(ch) = self.read_char(position) {
            if !ch.is_ascii_digit() {
                break;
            }
            position += 1;
        }
        position
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
    use crate::token::TokenType::{
        ASSIGN, ASTERISK, BANG, COMMA, ELSE, EOF, EQ, FALSE, FUNCTION, GT, IDENT, IF, INT, LBRACE,
        LET, LPAREN, LT, MINUS, NOTEQ, PLUS, RBRACE, RETURN, RPAREN, SEMICOLON, SLASH, TRUE,
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
            (NOTEQ, "!="),
            (INT, "9"),
            (SEMICOLON, ";"),
            (EOF, " "),
        ];

        let l = Lexer::new(input);
        let mut posiiton = 0;

        for (i, tt) in tests.into_iter().enumerate() {
            let (tok, read_position) = l.next_token(posiiton);
            if tok.token_type != tt.0 {
                panic!(
                    "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                    i, tt.0, tok.token_type
                );
            }

            if tok.literal != tt.1 {
                panic!(
                    "tests[{}] - Literal wrong. expected={}, got={}",
                    i, tt.1, tok.literal
                )
            }
            posiiton = read_position;
        }
    }
}
