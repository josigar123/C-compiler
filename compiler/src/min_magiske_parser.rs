use crate::token::{Token, TokenType};

pub enum Expr {
    Number(i32),
}

pub struct ExprNode {
    expr: Expr,
}

pub enum Statement {
    Return(ExprNode),
}

pub struct StatementNode {
    pub statement: Statement,
}

pub struct FunctionNode {
    pub return_value: TokenType,
    pub name: TokenType,
    pub body: Vec<StatementNode>,
}

pub struct Program {
    pub body: Vec<FunctionNode>,
}

// Eneste statement er foreløpig kun return immediate
// Indexet skal peke på første token i statement, skal da peke på return
pub fn parse_statement(
    tokens: &Vec<Token>,
    mut token_index: usize,
) -> Option<(StatementNode, usize)> {
    // Out of bounds
    if token_index >= tokens.len() {
        return None;
    }

    let statement_start = tokens.get(token_index).unwrap();

    match statement_start.token_type {
        TokenType::ReturnKeyword => {
            token_index += 1;
            // Vil nå parse statement kroppen, skal kun inneholde et heltall og en semi
            if let Some((expr_node, new_token_index)) = parse_expression(tokens, token_index) {
                token_index = new_token_index;
                // Må nå sjekke om semikolon er tilstede, eller err!
                let statement_end = tokens.get(token_index).unwrap();
                match statement_end.token_type {
                    TokenType::Semi => {
                        // Korrekt syntax, returner node
                        token_index += 1;
                        Some((
                            StatementNode {
                                statement: Statement::Return(expr_node),
                            },
                            token_index,
                        ))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

// token_index skal peke på første token av expression
pub fn parse_expression(tokens: &Vec<Token>, mut token_index: usize) -> Option<(ExprNode, usize)> {
    // Out of bounds
    if token_index >= tokens.len() {
        return None;
    }

    let expression_start = tokens.get(token_index).unwrap();
    match expression_start.token_type {
        TokenType::IntLit => {
            token_index += 1;
            let integer_as_string = expression_start.value.as_ref().unwrap();

            let parsed_int: i32 = match integer_as_string.parse() {
                Ok(parsed) => parsed,
                // Kan returne None for nå, men bør si ifra at heltall er forventet
                Err(_) => return None,
            };

            // Returnerer her en tuple med Expression noden og token_indexet
            // Indexet bør nå peke på semikolon
            Some((
                ExprNode {
                    expr: Expr::Number(parsed_int),
                },
                token_index,
            ))
        }
        _ => None,
    }
}

pub fn parse_function(tokens: &Vec<Token>, token_index: usize) -> Option<(FunctionNode, usize)> {
    if token_index >= tokens.len() {
        return None;
    }

    let function_start = tokens.get(token_index).unwrap();
    match function_start.token_type {
        TokenType::IntKeyword => {
            token_index += 1;
            // Vil nå parse for identifier
            let function_name_token = tokens.get(token_index).unwrap();
            let function_name = match &function_name_token.token_type {
                TokenType::Identifier => function_name_token.value.unwrap(),
                _ => None,
            };

            token_index += 1;
            // Vil nå parse for '(' ')', antar ingen args
            let peek_r_paren = tokens.get(token_index + 1).unwrap();
            let l_paren = tokens.get(token_index).unwrap();
        }
        _ => None,
    }
}
