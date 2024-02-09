use crate::token::{
    lookup_ident, Token,
    TokenType::{
        self, ASSIGN, ASTERISK, BANG, COMMA, EOF, EQ, GT, ILLEGAL, INT, LBRACE, LPAREN, LT, MINUS,
        NOTEQ, PLUS, RBRACE, RPAREN, SEMICOLON, SLASH,
    },
};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: b' ',
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::new();

        self.skip_whitespace();

        match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch as char, self.ch as char);
                    tok = Token {
                        token_type: EQ,
                        literal,
                    }
                } else {
                    tok = new_token(ASSIGN, self.ch);
                }
            }
            b'+' => tok = new_token(PLUS, self.ch),
            b'-' => tok = new_token(MINUS, self.ch),
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch as char, self.ch as char);
                    tok = Token {
                        token_type: NOTEQ,
                        literal,
                    }
                } else {
                    tok = new_token(BANG, self.ch);
                }
            }
            b'/' => tok = new_token(SLASH, self.ch),
            b'*' => tok = new_token(ASTERISK, self.ch),
            b'<' => tok = new_token(LT, self.ch),
            b'>' => tok = new_token(GT, self.ch),
            b';' => tok = new_token(SEMICOLON, self.ch),
            b',' => tok = new_token(COMMA, self.ch),
            b'{' => tok = new_token(LBRACE, self.ch),
            b'}' => tok = new_token(RBRACE, self.ch),
            b'(' => tok = new_token(LPAREN, self.ch),
            b')' => tok = new_token(RPAREN, self.ch),
            0 => {
                tok.literal = "".to_string();
                tok.token_type = EOF;
            }
            _ => {
                if is_letter(self.ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = lookup_ident(&tok.literal);
                    return tok;
                } else if self.ch.is_ascii_digit() {
                    tok.token_type = INT;
                    tok.literal = self.read_number();
                    return tok;
                } else {
                    tok = new_token(ILLEGAL, self.ch);
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        self.ch = self
            .input
            .chars()
            .nth(self.read_position)
            .map_or(0, |ch| ch as u8);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        self.input
            .chars()
            .nth(self.read_position)
            .map_or(0, |ch| ch as u8)
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let mut position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
}

fn new_token(token_type: TokenType, ch: u8) -> Token {
    Token {
        token_type,
        literal: (ch as char).to_string(),
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::Lexer;
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
        ";

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
            (EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for (i, tt) in tests.into_iter().enumerate() {
            let tok = l.next_token();

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
        }
    }
}
