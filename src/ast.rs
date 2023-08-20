use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
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

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
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
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

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

    fn expression_node() {}

    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
