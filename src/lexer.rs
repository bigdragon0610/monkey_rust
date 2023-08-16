use crate::token::{
    lookup_ident, Token, TokenType, ASSIGN, COMMA, ILLEGAL, INT, LBRACE, LPAREN, PLUS, RBRACE,
    RPAREN, SEMICOLON,
};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: impl Into<String>) -> Self {
        let mut l = Self {
            input: input.into(),
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if let Some(ch) = self.input.chars().nth(self.read_position) {
            self.ch = ch;
        } else {
            self.ch = ' ';
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => new_token(ASSIGN, self.ch),
            ';' => new_token(SEMICOLON, self.ch),
            '(' => new_token(LPAREN, self.ch),
            ')' => new_token(RPAREN, self.ch),
            ',' => new_token(COMMA, self.ch),
            '+' => new_token(PLUS, self.ch),
            '{' => new_token(LBRACE, self.ch),
            '}' => new_token(RBRACE, self.ch),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token {
                        token_type: lookup_ident(&literal),
                        literal,
                    };
                } else if self.ch.is_digit(10) {
                    return Token {
                        token_type: INT,
                        literal: self.read_number(),
                    };
                } else {
                    new_token(ILLEGAL, self.ch)
                }
            }
        };
        self.read_char();
        tok
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
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
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
        ASSIGN, COMMA, FUNCTION, IDENT, INT, LBRACE, LET, LPAREN, PLUS, RBRACE, RPAREN, SEMICOLON,
    };

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
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
        ];

        let mut l = Lexer::new(input);

        for (i, tt) in tests.into_iter().enumerate() {
            let tok = l.next_token();
            println!("{}", tok.literal);
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
