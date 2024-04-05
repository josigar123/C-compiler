use crate::lex;
use lex::Token;
use lex::TokenType;

//Veldig basic top-down recursive descent parser med utgangspunkt i figur 3.10 i engineering-a-compiler
//Med forbehold om endringer.

enum ASTNode {
    BinaryOp {
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Number(i32),
}

pub fn expr(tokens: &[Token], index: usize) -> Result<(usize, ASTNode), String> {
    // Expr -> Term Expr'
    let (new_index, node) = term(tokens, index)?; 
    e_prime(tokens, new_index, node)
} //Complete

pub fn term(tokens: &[Token], index: usize) -> Result<(usize, ASTNode), String> {
    // Term -> Factor Term'
    let (new_index, node) = factor(tokens, index)?; 
    t_prime(tokens, new_index, node)
} //Complete

pub fn e_prime(tokens: &[Token], index: usize, left_node: ASTNode) -> Result<(usize, ASTNode), String> {
    // Expr' --> + Term Expr'
    // Expr' --> - Term Expr'
    if let Some(token) = tokens.get(index) {
        match &token.value.as_deref() {
            Some(value) if value == "+" || value == "-" => {
                let op = token.value.clone().unwrap();
                let (new_index, right_node) = term(tokens, index + 1)?;
                let new_node = ASTNode::BinaryOp {
                    op: op,
                    left: Box::new(left_node),
                    right: Box::new(right_node),
                };
                e_prime(tokens, new_index, new_node)
            },
            _ => Ok((index, left_node)),

        }
    } else {
        Err("Unexpected input for EPrime".to_string())
    }
} //Complete

pub fn t_prime(tokens: &[Token], index: usize, left_node: ASTNode) -> Result<(usize, ASTNode), String> {
    // Term' -> x Factor Term'
    // Term ->  + Factor Term'
    if let Some(token) = tokens.get(index) {
        match &token.value {
            Some(value) if value == "x"  => {
                let (new_index, right_node) = factor(tokens, index + 1)?;
                let new_node = ASTNode::BinaryOp {
                    op: "x".to_string(),
                    left: Box::new(left_node),
                    right: Box::new(right_node),
                };
                t_prime(tokens, new_index, new_node)
            },
            Some(value) if value == "+" => {
                Ok((index, left_node))
            },
            _ => Ok((index, left_node)) // If EOF or tokens that end term
        }
    
    } else {
        Err("Unexpected input for TPrime".to_string())
    }

} //Complete

pub fn factor(tokens: &[Token], index: usize) -> Result<(usize, ASTNode), String> {
    // Factor -> (Expr)
    if let Some(token) = tokens.get(index) {
        match &token.value {
            Some(value) if value == "(" => {
                let (new_index, expr_node) = expr(tokens, index + 1)?;
                if let Some(next_token) = tokens.get(new_index) {
                    if next_token.value == Some(")".to_string()) {
                        return Ok((new_index + 1, expr_node));
                    }
                }
                Err("Expected ')'".to_string())
            },
            _ if token.token_type == TokenType::IntLit => {
                if let Ok(num) = value.parse::<i32>() {
                    return Ok((index + 1, ASTNode::Number(num)));
                }
                Err("Invalid integer literal".to_string())
            },
            _ => Err("Unexpected token in factor".to_string()),
        }
    } else {
        Err("No token found in factor".to_string());
    }
} //Complete

pub fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    let mut index = 0;
    match expr(&tokens, index) {
        Ok((_, ast)) => Ok(ast),
        Err(err) => {
            Err(err)
        }
    }
}