use crate::lex;
use lex::Token;
use lex::TokenType;

//Veldig basic top-down recursive descent parser med utgangspunkt i figur 3.10 i engineering-a-compiler
//Med forbehold om endringer.

pub fn fail(tokens: &[Token]) { //Må refaktoreres
    println!("Error" );
}

pub fn expr(tokens: &[Token], index: usize) -> Result<usize, String> {
    // Expr -> Term Expr'
    if let Ok(new_index) = term(tokens, index) {
        e_prime(tokens, new_index)
    } else {
        Err("Failed to parse expression".to_string())
    }
}

pub fn term(tokens: &[Token], index: usize) -> Result<usize, String> {
    // Term -> Factor Term'
    if let Ok(new_index) = factor(tokens, index) {
        t_prime(tokens, new_index)
    } else {
        Err("Failed to parse term".to_string())
    }
}

pub fn e_prime(tokens: &[Token], index: usize) -> Result<usize, String> {
    // Expr' --> + Term Expr'
    // Expr' --> - Term Expr'
    if let Some(token) = tokens.get(index) {
        if let Some(ref value) = token.value {
            if value == "+" || value == "-" {
                let new_index = index + 1;
                let new_index = term(tokens, new_index)?;
                return e_prime(tokens, new_index);
            } else if value == ")" || index >= tokens.len() {
                return Ok(index);
            }
        }
    }
    Err("Unexpected input for EPrime".to_string())
}

pub fn t_prime(tokens: &[Token], index: usize) -> Result<usize, String> {
    // Term' -> x Factor Term'
    // Term ->  + Factor Term'
    if let Some(token) = tokens.get(index) {
        if let Some(ref value) = token.value {
            if value == "x" || value == "+" {
                let new_index = index + 1;
                let new_index = factor(tokens, new_index)?;
                return t_prime(tokens, new_index);
            } else if value == "=" || index >= tokens.len() || value == "-" || value == ")" {
                return Ok(index);
            }
        }
    }
    Err("Unexpected input for TPrime".to_string())
}

pub fn factor(tokens: &[Token], index: usize) -> Result<usize, String> {
    // Factor -> (Expr)
    if let Some(token) = tokens.get(index) {
        if let Some(ref value) = token.value {
            if value == "(" {
                let new_index = index + 1;
                let new_index = expr(tokens, new_index)?;
                if let Some(next_token) = tokens.get(new_index) { //Noen logikkfeil her nede.
                    if next_token.value == Some(")".to_string()) {
                        return Ok(new_index + 1);
                    }
                }
            } else if let TokenType::IntLit = token.token_type {
                return Ok(index + 1);
            }
        }
    }
    Err("Failed to parse factor".to_string())
}

pub fn parse(tokens: Vec<Token>) -> Result<(), String> {
    let mut index = 0;
    println!("Du er her");
    match expr(&tokens, index) {
        Ok(_) => Ok(()),
        Err(err) => {
            fail(&tokens);
            Err(err)
        }
    }
}