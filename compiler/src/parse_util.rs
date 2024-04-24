use crate::parser::Parser;
use crate::token::{Token, TokenType};

impl Parser {
    // Consume a token from token stream, øker index "Konsumerer"
    pub fn consume(&mut self) {
        self.token_index += 1;
    }

    // Peek en token, returnerer den, advancer ikke token_stream
    pub fn peek(&mut self, offset: usize) -> Option<&Token> {
        if self.token_index + offset >= self.token_stream.len() {
            return None;
        }
        return self.token_stream.get(self.token_index + offset);
    }

    // Forvent token, e.g ved funksjoner forventes en struktur
    pub fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        // Checks the current token
        match self.peek(0) {
            Some(token) if token.token_type == expected => Ok(()),
            Some(token) => Err(format!("Expected {:?} but found {:?}", expected, token)),
            None => Err("Unexpected stream end".to_string()),
        }
    }
}
