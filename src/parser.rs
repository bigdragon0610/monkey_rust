use crate::{
    ast::{Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    l: Lexer,
    errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        Parser {
            l,
            errors: Vec::new(),
            cur_token: Token::new(),
            peek_token: Token::new(),
        }
    }

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.cur_token.token_type
        );
        self.errors.push(msg);
    }

    fn next_token(&mut self, position: usize) -> usize {
        self.cur_token = self.peek_token.clone();
        let (tok, position) = self.l.next_token(position);
        self.peek_token = tok;
        position
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        let mut position = 0;

        while self.cur_token.token_type != TokenType::EOF {
            if let (Some(stmt), expect_position) = self.parse_statement(position) {
                program.statements.push(stmt);
                position = expect_position;
            }
            position = self.next_token(position);
        }
        program
    }

    fn parse_statement(&mut self, position: usize) -> (Option<Statement>, usize) {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(position),
            TokenType::RETURN => self.parse_return_statement(position),
            _ => (None, position),
        }
    }

    fn parse_let_statement(&mut self, position: usize) -> (Option<Statement>, usize) {
        let mut stmt = LetStatement::new();
        stmt.token = self.cur_token.clone();
        let mut position = position;

        if let Some(expect_position) = self.expect_peek(TokenType::IDENT, position) {
            position = expect_position;
        } else {
            return (None, position);
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if let Some(expect_position) = self.expect_peek(TokenType::ASSIGN, position) {
            position = expect_position;
        } else {
            return (None, position);
        }

        while self.cur_token_is(TokenType::SEMICOLON) {
            position = self.next_token(position);
        }

        (Some(Statement::LetStatement(stmt)), position)
    }

    fn parse_return_statement(&mut self, position: usize) -> (Option<Statement>, usize) {
        let stmt = ReturnStatement::new(self.cur_token.clone());

        let mut position = position;
        self.next_token(position);

        while !self.cur_token_is(TokenType::SEMICOLON) {
            position = self.next_token(position);
        }

        (Some(Statement::ReturnStatement(stmt)), position)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType, position: usize) -> Option<usize> {
        if self.peek_token_is(t) {
            let position = self.next_token(position);
            return Some(position);
        } else {
            self.peek_error(t);
            return None;
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
        let mut p = Parser::new(l);

        let program = p.parse_program();
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
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got={}",
                program.statements.len()
            );
        }

        println!("{:?}", program);
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
