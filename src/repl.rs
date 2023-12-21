use std::io::Write;

use crate::{lexer::Lexer, token::TokenType::EOF};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut buf = String::new();
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut l = Lexer::new(buf.clone());
        loop {
            let tok = l.next_token();
            if tok.token_type == EOF {
                break;
            }
            println!("token_type:{:?} literal:{}", tok.token_type, tok.literal);
        }
        buf.clear()
    }
}
