use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program,
        ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

enum Operator {
    Lowest,
    Equals,
    LessGrater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug, Clone)]
struct Parser {
    l: Lexer,
    errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            errors: Vec::new(),
            cur_token: Token::new(),
            peek_token: Token::new(),
        };

        p.next_token();
        p.next_token();

        p
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&self, t: TokenType) -> String {
        format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        )
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.l.next_token())
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let mut stmt = LetStatement::new();
        stmt.token = self.cur_token.clone();

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::LetStatement(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let stmt = ReturnStatement::new(self.cur_token.clone());

        self.next_token();

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::ReturnStatement(stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Operator::Lowest as usize),
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::ExpressionStatement(stmt))
    }

    fn parse_expression(&mut self, precedence: usize) -> Option<Expression> {
        match self.cur_token.token_type {
            TokenType::IDENT => self.parse_identifier(),
            TokenType::INT => self.parese_integer_literal(),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parese_integer_literal(&mut self) -> Option<Expression> {
        let value = match self.cur_token.literal.parse() {
            Ok(value) => value,
            _ => {
                self.errors.push(format!(
                    "could not parse {} as integer",
                    self.cur_token.literal
                ));
                return None;
            }
        };

        let lit = IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        };

        Some(Expression::IntegerLiteral(lit))
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, Node, Statement},
        lexer::Lexer,
    };

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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        if program.statements.len() != 1 {
            panic!(
                "program has not enough statements. got={}",
                program.statements.len(),
            );
        }

        let stmt = match &program.statements[0] {
            Statement::ExpressionStatement(stmt) => stmt,
            _ => panic!(
                "program.statements[0] is not Statement::ExpressionStatement. got={:?}",
                program.statements[0]
            ),
        };

        let ident = match &stmt.expression {
            Some(Expression::Identifier(ident)) => ident,
            _ => panic!("exp not Expression::Identifier. got={:?}", stmt.expression),
        };
        if ident.value != "foobar" {
            panic!("ident.value not {}. got={}", "foobar", ident.value);
        }
        if ident.token_literal() != "foobar" {
            panic!(
                "ident.token_literal not {}. got={}",
                "foobar",
                ident.token_literal()
            );
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 1 {
            panic!(
                "program has no enough statements. got={}",
                program.statements.len()
            );
        }
        let stmt = match &program.statements[0] {
            Statement::ExpressionStatement(stmt) => stmt,
            _ => panic!(
                "program.statements[0] is not Statement::ExpressionStatement. got={:?}",
                program.statements[0]
            ),
        };

        let literal = match &stmt.expression {
            Some(Expression::IntegerLiteral(literal)) => literal,
            _ => panic!(
                "exp not Expression::IntegerLiteral. got={:?}",
                stmt.expression
            ),
        };
        if literal.value != 5 {
            panic!("literal.value not {}. got={}", 5, literal.value);
        }
        if literal.token_literal() != "5" {
            panic!(
                "literal.token_literal not {}. got={}",
                5,
                literal.token_literal()
            );
        }
    }
}
