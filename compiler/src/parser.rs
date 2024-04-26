use crate::token::{Token, TokenType};

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

    // Expr er nå kun i32
    fn parse_expression(&mut self) -> Option<ExprNode> {
        // Bekrefter at token vil ha en verdi
        if self.token_index >= self.token_stream.len() {
            return None;
        }

        let current_token = self.peek(0).expect("Token is None");

        match current_token.token_type {
            TokenType::IntLit => self.parse_integer(),
            TokenType::BitComplement | TokenType::Minus | TokenType::Not => {
                self.parse_unary_operation()
            }
            _ => None,
        }
    }

    fn parse_term(&mut self) -> Option<ExprNode> {
        // Parser for factor
        let factor = self.parse_factor();

        // Placeholder value
        let mut complete_factor = factor.clone();
        // Skal peke på neste token i stream
        let mut current_token = self.peek(0).expect("Token is None");
        // Vil parse så lenge det er enten  * || /
        while current_token.token_type == TokenType::Mul
            || current_token.token_type == TokenType::Div
        {
            let operator = current_token.token_type.clone();
            self.consume(); // Spiser enten * || /
            let next_factor = self.parse_factor();
            complete_factor = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(factor.clone().expect("Factor failed")),
                    Box::new(next_factor.expect("Next factor failed")),
                ),
            });
            current_token = self.peek(0).expect("Token is None");
        }

        complete_factor
    }

    fn parse_factor(&mut self) -> Option<ExprNode> {
        /*En faktor kan være:
           (expr),
           UnaryOp,
           Int
        */
        // Current tok vi kan matche på
        let current_token = self.peek(0).expect("Token is None");

        match current_token.token_type {
            // "(" <expr> ")" case
            TokenType::LParen => {
                self.consume(); // Consume '(' token
                let expression = self.parse_expression_refactor(); // Parse for nested expression
                self.consume(); // Consume expression node

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
                self.parse_unary_operation()
            }
            // IntLit case
            TokenType::IntLit => self.parse_integer(),
            _ => {
                println!("Expected factor, founc {:?}", current_token.token_type);
                None
            }
        }
    }

    fn parse_expression_refactor(&mut self) -> Option<ExprNode> {
        unimplemented!();
    }

    fn parse_unary_operation(&mut self) -> Option<ExprNode> {
        let current_token = self.peek(0).expect("Token is None");
        match current_token.token_type {
            TokenType::BitComplement | TokenType::Minus | TokenType::Not => {
                // Current op: ~, - || !
                let operator = current_token.clone();

                // Consume operand
                self.consume();

                // Want to parse the expression recursively
                let operand = self.parse_expression();

                // Create expression node
                Some(ExprNode {
                    expr: Expr::UnaryOp(operator.token_type, operand.map(Box::new)),
                })
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
