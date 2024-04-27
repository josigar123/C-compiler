use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i32),
    UnaryOp(TokenType, Option<Box<ExprNode>>),
    BinaryOp(TokenType, Box<ExprNode>, Box<ExprNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprNode {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Return(ExprNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub statement: Statement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub return_value: TokenType,
    pub name: String, // Kan evt være expected tokentype Ident
    pub body: Vec<StatementNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramNode {
    pub body: Vec<FunctionNode>,
}

pub struct Parser {
    pub token_index: usize,
    pub token_stream: Vec<Token>,
}

impl Parser {
    // Constructor, implicitly sets index to 0
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_index: 0,
            token_stream: tokens,
        }
    }

    fn parse_term(&mut self) -> Option<ExprNode> {
        // Parser for factor
        println!("parse_term: parsing for a factor");
        let factor = self.parse_factor();

        println!("parse_term: parsed factor: {:?}", factor);

        let mut complete_factor = factor.clone();
        // Skal peke på neste token i stream
        let mut current_token = self.peek(0).expect("Token is None");
        println!("parse_term: current_token: {:?}", current_token);
        // Vil parse så lenge det er enten  * || /
        while current_token.token_type == TokenType::Mul
            || current_token.token_type == TokenType::Div
        {
            let operator = current_token.token_type.clone();
            println!("parse_term: operator: {}", operator);
            self.consume(); // Spiser enten * || /
            println!("parse_term: parsing for next_factor");
            let next_factor = self.parse_factor();
            println!("parse_term: parsed next_factor {:?}", next_factor);

            let new_factor = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(complete_factor.clone().expect("Incomplete term")),
                    Box::new(next_factor.expect("Next term failed")),
                ),
            });
            complete_factor = new_factor.clone();

            println!("parse_term: complete_factor: {:?}", complete_factor);
            current_token = self.peek(0).expect("Token is None");
        }

        complete_factor
    }

    fn parse_factor(&mut self) -> Option<ExprNode> {
        // Current tok vi kan matche på
        let current_token = self.peek(0).expect("Token is None");
        println!("parse_factor: current_token: {:?}", current_token);
        match current_token.token_type {
            // "(" <expr> ")" case
            TokenType::LParen => {
                println!("Token is ( , parsing nested expression");
                self.consume(); // Consume '(' token
                let expression = self.parse_expression(); // Parse for nested expression

                // Should expect ')'
                if let Err(error) = self.expect(TokenType::RParen) {
                    println!("Error {}", error);
                    return None;
                }

                // Consumer ')'
                self.consume();
                expression
            }
            // Unary Op case
            TokenType::BitComplement | TokenType::Minus | TokenType::Not => {
                println!("Factor has a unary operator, parsing for it");
                self.parse_unary_operation()
            }
            // IntLit case
            TokenType::IntLit => {
                println!("Factor is an IntLit, parsing for it");
                self.parse_integer()
            }
            _ => {
                println!("Expected factor, found {:?}", current_token.token_type);
                None
            }
        }
    }

    fn parse_expression(&mut self) -> Option<ExprNode> {
        println!("parse_expression: Parsing for term in expression");
        let term = self.parse_term();

        println!("parse_expression: first parsed term: {:?}", term);
        // Placeholder
        let mut complete_term = term.clone();

        // Skal peke på første token i en expr
        let mut current_token = self.peek(0).expect("Token is None");
        println!("parse_expression: current_token: {:?}", current_token);
        // Another term while this is true
        while current_token.token_type == TokenType::Plus
            || current_token.token_type == TokenType::Minus
        {
            let operator = current_token.token_type.clone();
            println!("parse_expression: operator: {:?}", operator);
            // Advance stream
            self.consume();
            println!("parse_expression: parsing for next term");
            let next_term = self.parse_term();
            println!("parse_expression: next_term: {:?}", next_term);
            println!("parse_expression: term to add to complete term: {:?}", term);

            let new_term = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(complete_term.clone().expect("Incomplete term")),
                    Box::new(next_term.expect("Next term failed")),
                ),
            });
            complete_term = new_term.clone();
            println!("parse_expression: complete_term: {:?}", complete_term);
            current_token = self.peek(0).expect("Token is None");
        }

        complete_term
    }

    fn parse_unary_operation(&mut self) -> Option<ExprNode> {
        println!("parse_unary_opearation: parsing for UnOp");
        let current_token = self.peek(0).expect("Token is None");
        println!("parse_unary_opearation: current_token: {:?}", current_token);
        match current_token.token_type {
            TokenType::BitComplement | TokenType::Minus | TokenType::Not => {
                // Current op: ~, - || !
                let operator = current_token.clone();

                println!("parse_unary_opearation: operator: {:?}", operator);
                // Consume operator
                self.consume();

                // Want to parse the expression recursively
                println!("parse_unary_opearation: parsing for operand");
                let operand = self.parse_expression();
                println!("parse_unary_opearation: operand: {:?}", operand);
                // Create expression node
                let unary_operator = Some(ExprNode {
                    expr: Expr::UnaryOp(operator.token_type, operand.map(Box::new)),
                });
                println!(
                    "parse_unary_opearation: unary_operator: {:?}",
                    unary_operator
                );
                unary_operator
            }
            _ => {
                println!(
                    "Error: Expected a unary operator, found {:?}",
                    current_token.token_type
                );
                None
            }
        }
    }
    fn parse_integer(&mut self) -> Option<ExprNode> {
        println!("parse_integer: parsing for int");
        // Forventer at Expr skal være et heltall
        if let Err(error) = self.expect(TokenType::IntLit) {
            println!("Error {}", error);
            return None;
        }

        let int_value = self.token_stream[self.token_index]
            .value
            .as_ref()
            .map(|s| {
                s.parse::<i32>().map_err(|e| {
                    println!("Error: Failed to parse integer: {}", e);
                })
            })
            .and_then(|result| result.ok());

        let parsed = match int_value {
            Some(parsed) => parsed,
            None => return None,
        };

        println!("parse_integer: parsed: {:?}", parsed);
        // spiser expression
        self.consume();
        Some(ExprNode {
            expr: Expr::Number(parsed),
        })
    }

    fn parse_statement(&mut self) -> Option<StatementNode> {
        if self.token_index > self.token_stream.len() {
            return None;
        }

        // Forventer return da dette er eneste expression
        if let Err(error) = self.expect(TokenType::ReturnKeyword) {
            println!("Error {}", error);
            return None;
        }

        // Move into expression
        self.consume();
        let expression;
        if let Some(statement_expression) = self.parse_expression() {
            expression = statement_expression;
        } else {
            println!("Error: Failed to parse expression");
            return None;
        }

        // Neste token er forventet å være semikolon
        if let Err(error) = self.expect(TokenType::Semi) {
            println!("Error {}", error);
            return None;
        }

        // Spiser semikolon
        self.consume();

        Some(StatementNode {
            statement: Statement::Return(expression),
        })
    }

    fn parse_function(&mut self) -> Option<FunctionNode> {
        if self.token_index >= self.token_stream.len() {
            return None;
        }

        // Holder statements
        let mut statement_list: Vec<StatementNode> = vec![];

        // Expect IntKeyword
        if let Err(error) = self.expect(TokenType::IntKeyword) {
            println!("Error {}", error);
            return None;
        } // int

        let return_type = self.token_stream[self.token_index].token_type.clone();

        // Spiser returtype
        self.consume();

        if let Err(error) = self.expect(TokenType::Identifier) {
            println!("Error {}", error);
            return None;
        } // main or other function ident

        let function_name = match self.token_stream.get(self.token_index) {
            Some(token) => match &token.value {
                Some(value) => value.clone(),
                None => {
                    println!("Error: Missing function name");
                    return None;
                }
            },
            None => {
                println!("Error: Token index out of range");
                return None;
            }
        };

        // Consume Identifier
        self.consume();
        if let Err(error) = self.expect(TokenType::LParen) {
            println!("Error {}", error);
            return None;
        } // (
          // Consume LParen
        self.consume();
        if let Err(error) = self.expect(TokenType::RParen) {
            println!("Error {}", error);
            return None;
        } // )
          // Consume RParen
        self.consume();
        if let Err(error) = self.expect(TokenType::LBrace) {
            println!("Error {}", error);
            return None;
        } // {
          // Consume LBrace
        self.consume();

        // For flere statements så må det være en løkke som pusher alle statements på listen
        let statement = self.parse_statement();

        if let Err(error) = self.expect(TokenType::RBrace) {
            println!("Error {}", error);
            return None;
        } // }

        self.consume(); // Consume }

        // Kun 1 statement for nå
        // Verdt å refaktorere unrap bruken her
        statement_list.push(statement.unwrap());
        Some(FunctionNode {
            return_value: return_type,
            name: function_name,
            body: statement_list,
        })
    }

    pub fn parse_program(&mut self) -> Option<ProgramNode> {
        if self.token_index > self.token_stream.len() {
            return None;
        }

        let mut function_list: Vec<FunctionNode> = vec![];

        while let Some(function) = self.parse_function() {
            function_list.push(function);
        }

        Some(ProgramNode {
            body: function_list,
        })
    }
}
