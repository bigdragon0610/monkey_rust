use crate::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn token_literal(&self) -> String {
        self.statements
            .first()
            .map_or("".to_string(), |s| s.token_literal())
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Self::LetStatement(let_stmt) => let_stmt.token.literal.clone(),
            Self::ReturnStatement(return_stmt) => return_stmt.token.literal.clone(),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl LetStatement {
    pub fn new() -> Self {
        Self {
            token: Token::new(),
            name: Identifier::new(),
            value: None,
        }
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            return_value: None,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Self::Identifier(identifier) => identifier.token.literal.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    fn new() -> Self {
        Self {
            token: Token::new(),
            value: String::default(),
        }
    }

    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
