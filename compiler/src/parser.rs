use crate::lex;
use lex::Token;
use lex::TokenType;

pub enum Expression {
    Number(i32),
    Name(String),
    BinaryOp {
        op: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assign { 
        name: String, 
        value: Box<Expression>
     },
}

pub enum Statement { //Ikke implementert metodene for parsing av dette enda.
    Expression(Expression),
    Return(Expression),
    If { condition: Expression, then_branch: Box<Statement>, else_branch: Option<Box<Statement>>},
    While { condition: Expression, body: Box<Statement> },
}


pub fn expr(tokens: &[Token], index: usize) -> Result<(usize, Expression), String> {
    // Expr -> Term Expr'
    let (new_index, node) = term(tokens, index)?; // returner Err hvis det er feil ved eksekveringen av koden. Videre linjer blir ikke eksekvert.
    e_prime(tokens, new_index, node)
} //Complete

pub fn term(tokens: &[Token], index: usize) -> Result<(usize, Expression), String> {
    // Term -> Factor Term'
    let (new_index, node) = factor(tokens, index)?; // returner Err hvis det er feil ved eksekveringen av koden. Videre linjer blir ikke eksekvert.
    t_prime(tokens, new_index, node)
} //Complete


pub fn e_prime(tokens: &[Token], index: usize, left_node: Expression) -> Result<(usize, Expression), String> {
    if let Some(token) = tokens.get(index) {
        match token.token_type {
            TokenType::Plus | TokenType::Minus => {
                let op = if token.token_type == TokenType::Plus { "+" } else { "-" };
                let (new_index, right_node) = term(tokens, index + 1)?;
                let new_node = Expression::BinaryOp {
                    op: op.to_string(),
                    left: Box::new(left_node),
                    right: Box::new(right_node),
                };
                e_prime(tokens, new_index, new_node)
            },
            _ => Ok((index, left_node)),
        }
    } else {
        Ok((index, left_node));
        //Err("Unexpected end of input in EPrime'".to_string())
    }
}//Complete


pub fn t_prime(tokens: &[Token], index: usize, left_node: Expression) -> Result<(usize, Expression), String> {
    if let Some(token) = tokens.get(index) {
        match token.token_type {
            TokenType::Mul | TokenType::Div => {
                let op = if token.token_type == TokenType::Mul { "*" } else { "/" };
                let (new_index, right_node) = factor(tokens, index + 1)?;
                let new_node = Expression::BinaryOp {
                    op: op.to_string(),
                    left: Box::new(left_node),
                    right: Box::new(right_node),
                };
                t_prime(tokens, new_index, new_node)
            },
            _ => Ok((index, left_node)),
        }
    } else {
        Ok((index, left_node))
        //Err("Unexpected end of input in TPrime'".to_string())
    }
} //Complete

pub fn factor(tokens: &[Token], index: usize) -> Result<(usize, Expression), String> {
    // Factor -> (Expr)
    if let Some(token) = tokens.get(index) {
        match &token.token_type {
            Some(token) if token.token_type == TokenType::LParen => {    //Dersom uttrykket begynner med ( vil vi se hva som er innholdet i parantene, det kan være hva 
                
                let (expr_node, new_index) = expr(tokens, index + 1)?;
                if let Some(next_token) = tokens.get(new_index) {
                    if next_token.token_type == TokenType::RParen {
                            return Ok((new_index + 1, expr_node));
                    }                    
                }
                Err("Expected ')'".to_string())
            },
            _ if token.token_type == TokenType::IntLit => { //Sjekker om det er intlit
                if let Ok(num) = token.parse::<i32>() {
                    return Ok((index + 1, Expression::Number(num)));
                }
                Err("Invalid integer literal".to_string())
            },
            _ if token.token_type == TokenType::StringLit => { //sjekker om det er stringlit
                if let Some(next_token) = tokens.get(index + 1) {
                    if token.token_type == TokenType::Assign { //Sjekker om det er en assignment operasjon
                        let (new_index, new_right_node) = expr(tokens, index + 2)?;                                 //Tilsvarer A = 5;
                        return Ok((new_index, Expression::Assign {name: token, value: Box::new(new_right_node) })) // Returnerer en ny indeks, og legger til en ny node i treet med verdien til variablen.
                    }                           
                }
                return Ok((index + 1, Expression::Name(token)))
            }
            _ => Err("Unexpected token in factor".to_string()),
        }

    } else {
        Err("No token found in factor".to_string());
    }
} //Complete

pub fn parse(tokens: Vec<Token>) -> Result<Expression, String> {
    let (index, ast) = expr(&tokens, 0)?;
    if index != tokens.len() {
        Err("Extra tokens after valid expression".to_string())
    } else {
        Ok(ast)
    }
}