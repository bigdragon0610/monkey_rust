use crate::{
    ast::{Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub struct Parser {
    l: Lexer,
    errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,
    position: usize,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        Parser {
            l,
            errors: Vec::new(),
            cur_token: Token::new(),
            peek_token: Token::new(),
            position: 0,
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&self, t: TokenType) -> String {
        format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        )
    }

    fn next_token(&self) -> Self {
        let mut parser = self.clone();
        parser.cur_token = parser.peek_token.clone();
        (parser.peek_token, parser.position) = parser.l.next_token(parser.position);
        parser
    }

    pub fn parse_program(&self) -> (Program, Parser) {
        let mut program = Program::new();
        let mut parser = self.clone();

        while parser.cur_token.token_type != TokenType::EOF {
            let (expect_parser, stmt) = parser.parse_statement();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            parser = expect_parser;
            parser = parser.next_token();
        }
        (program, parser)
    }

    fn parse_statement(&self) -> (Self, Option<Statement>) {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => (self.clone(), None),
        }
    }

    fn parse_let_statement(&self) -> (Self, Option<Statement>) {
        let mut parser = self.clone();
        let mut stmt = LetStatement::new();
        stmt.token = parser.cur_token.clone();

        match parser.expect_peek(TokenType::IDENT) {
            Ok(expect_parser) => parser = expect_parser,
            Err(error) => {
                parser.errors.push(error);
                return (parser, None);
            }
        }

        stmt.name = Identifier {
            token: parser.cur_token.clone(),
            value: parser.cur_token.literal.clone(),
        };

        match parser.expect_peek(TokenType::ASSIGN) {
            Ok(expect_parser) => parser = expect_parser,
            Err(error) => {
                parser.errors.push(error);
                return (parser, None);
            }
        }

        while !parser.cur_token_is(TokenType::SEMICOLON) {
            parser = parser.next_token();
        }

        (parser, Some(Statement::LetStatement(stmt)))
    }

    fn parse_return_statement(&self) -> (Self, Option<Statement>) {
        let mut parser = self.clone();
        let stmt = ReturnStatement::new(parser.cur_token.clone());

        parser = parser.next_token();

        while !parser.cur_token_is(TokenType::SEMICOLON) {
            parser = parser.next_token();
        }

        (parser, Some(Statement::ReturnStatement(stmt)))
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&self, t: TokenType) -> Result<Parser, String> {
        if self.peek_token_is(t) {
            Ok(self.next_token())
        } else {
            Err(self.peek_error(t))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Statement, lexer::Lexer};

    use super::Parser;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "
        .to_string();

        let l = Lexer::new(input);
        let p = Parser::new(l);

        let (program, p) = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got={}",
                program.statements.len()
            );
        }

        let tests = ["x", "y", "foobar"];
        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            if !test_let_statement(stmt, tt) {
                return;
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 993322;
        "
        .to_string();

        let l = Lexer::new(input);
        let p = Parser::new(l);

        let (program, p) = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got={}",
                program.statements.len()
            );
        }

        for stmt in program.statements {
            match stmt {
                Statement::ReturnStatement(_) => {
                    if stmt.token_literal() != "return" {
                        panic!(
                            "stmt.token_literal not 'return', got {}",
                            stmt.token_literal()
                        );
                    }
                }
                _ => panic!("stmt not ReturnStatement. got={:?}", stmt),
            }
        }
    }

    fn check_parser_errors(p: &Parser) {
        let errors = p.errors();
        if errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", errors.len());
        for msg in errors {
            println!("parser error: {}", msg);
        }
        panic!();
    }

    fn test_let_statement(s: &Statement, name: &str) -> bool {
        if s.token_literal() != "let" {
            println!("s.token_literal not 'let'. got={}", s.token_literal());
            return false;
        }

        if let Statement::LetStatement(let_stmt) = s {
            if let_stmt.name.value != name {
                println!(
                    "let_stmt.name.value not '{}'. got={}",
                    name, let_stmt.name.value
                );
                return false;
            }

            if let_stmt.name.token_literal() != name {
                println!(
                    "let_stmt.name.token_literal() not '{}', got={}",
                    name,
                    let_stmt.name.token_literal()
                );
                return false;
            }
        } else {
            println!("s not LetStatement. got={:?}", s);
            return false;
        }

        true
    }
}
