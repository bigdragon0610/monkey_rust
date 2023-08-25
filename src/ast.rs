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

    pub fn string(&self) -> String {
        self.statements.iter().map(|s| s.string()).collect()
    }
}

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::LetStatement(let_stmt) => let_stmt.token_literal(),
            Self::ReturnStatement(return_stmt) => return_stmt.token_literal(),
            Self::ExpressionStatement(expression_stmt) => expression_stmt.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Self::LetStatement(let_stmt) => let_stmt.string(),
            Self::ReturnStatement(return_stmt) => return_stmt.string(),
            Self::ExpressionStatement(expression_stmt) => expression_stmt.string(),
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

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value.clone().map_or("".to_string(), |v| v.string())
        )
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

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "{} {};",
            self.token_literal(),
            self.return_value
                .clone()
                .map_or("".to_string(), |v| v.string())
        )
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.expression
            .clone()
            .map_or("".to_string(), |v| v.string())
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::Identifier(identifier) => identifier.token.literal.clone(),
        }
    }

    fn string(&self) -> String {
        match self {
            Self::Identifier(identifier) => identifier.string(),
        }
    }
}

#[derive(Debug, Clone)]
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
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}
