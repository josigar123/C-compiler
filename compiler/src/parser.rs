use crate::symbol_table::{SymbolTable, TableEntry};
use crate::token::{Token, TokenType};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i32),
    Identifier(String),
    UnaryOp(TokenType, Option<Box<ExprNode>>),
    BinaryOp(TokenType, Box<ExprNode>, Box<ExprNode>),
    DeclAssign(
        Option<Box<ExprNode>>,
        Option<TokenType>,
        Option<Box<ExprNode>>,
    ), // a = 2 | 5 | a = (b=2) +3
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprNode {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Return(ExprNode),                                                       // return 2;
    Assignment(TokenType, Token, Option<TokenType>, Option<Box<ExprNode>>), // int a = 2;
    DeclAssignForStmnt(Option<Box<ExprNode>>),
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
    pub function_names: Vec<String>, // Stores all function names in program
    pub body: Vec<FunctionNode>,
}

pub struct Parser {
    pub token_index: usize,
    pub token_stream: Vec<Token>,
    pub symbol_table: Arc<Mutex<SymbolTable>>,
}

impl Parser {
    // Constructor, implicitly sets index to 0
    pub fn new(tokens: Vec<Token>, symbol_table_thread_safe: Arc<Mutex<SymbolTable>>) -> Self {
        Parser {
            token_index: 0,
            token_stream: tokens,
            symbol_table: symbol_table_thread_safe,
        }
    }

    // Function to increase readability, and maintain contract
    fn parse_expression(&mut self) -> Option<ExprNode> {
        self.parse_decl_assign()
    }

    fn parse_decl_assign(&mut self) -> Option<ExprNode> {
        let current_token = self.peek(0).expect("Token is None");
        let next_token_in_stream = self.peek(1).unwrap();
        if current_token.token_type == TokenType::Identifier
            && next_token_in_stream.token_type == TokenType::Assign
        {
            let identifier = self.parse_identifier(); // Function parses ident, consumes into next token

            // Add identifier to symbol table

            let next_token = self.peek(0).expect("Token is None").token_type.clone();
            let operator = next_token.clone();

            self.consume(); // Consume '=' or other operator

            let expression = self.parse_decl_assign();

            Some(ExprNode {
                expr: Expr::DeclAssign(
                    identifier.map(Box::new),
                    Some(operator),
                    expression.map(Box::new),
                ),
            })
        } else {
            self.parse_or()
        }
    }

    fn parse_or(&mut self) -> Option<ExprNode> {
        let and = self.parse_and();

        let mut complete_and = and.clone();

        let mut current_token = self.peek(0).expect("Token is None");
        while current_token.token_type == TokenType::Or {
            let operator = current_token.token_type.clone();
            self.consume();
            let next_and = self.parse_and();

            let new_and = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(complete_and.clone().expect("Incomplete and part")),
                    Box::new(next_and.expect("Next and part failed")),
                ),
            });

            complete_and = new_and.clone();
            current_token = self.peek(0).expect("Token is None");
        }

        complete_and
    }

    fn parse_and(&mut self) -> Option<ExprNode> {
        let eqality = self.parse_eqality();

        let mut complete_equality = eqality.clone();

        let mut current_token = self.peek(0).expect("Token is None");
        while current_token.token_type == TokenType::And {
            let operator = current_token.token_type.clone();
            self.consume();
            let next_equality = self.parse_eqality();

            let new_equality = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(complete_equality.clone().expect("Incomplete equality part")),
                    Box::new(next_equality.expect("Next equality part failed")),
                ),
            });

            complete_equality = new_equality.clone();
            current_token = self.peek(0).expect("Token is None");
        }

        complete_equality
    }

    fn parse_eqality(&mut self) -> Option<ExprNode> {
        let relational = self.parse_relation();

        let mut complete_relational = relational.clone();

        let mut current_token = self.peek(0).expect("Token is None");
        while current_token.token_type == TokenType::Eq
            || current_token.token_type == TokenType::Neq
        {
            let operator = current_token.token_type.clone();
            self.consume();
            let next_relational = self.parse_relation();

            let new_relational = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(
                        complete_relational
                            .clone()
                            .expect("Incomplete relational part"),
                    ),
                    Box::new(next_relational.expect("Next relational part failed")),
                ),
            });

            complete_relational = new_relational.clone();
            current_token = self.peek(0).expect("Token is None");
        }

        complete_relational
    }

    fn parse_relation(&mut self) -> Option<ExprNode> {
        let additive = self.parse_add();
        let mut complete_additive = additive.clone();

        let mut current_token = self.peek(0).expect("Token is None");
        while current_token.token_type == TokenType::Lt
            || current_token.token_type == TokenType::Gt
            || current_token.token_type == TokenType::Le
            || current_token.token_type == TokenType::Ge
        {
            let operator = current_token.token_type.clone();
            self.consume(); // consume operator

            let next_additive = self.parse_add();
            let new_additive = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(complete_additive.clone().expect("Incomplete additive part")),
                    Box::new(next_additive.expect("Next additive part failed")),
                ),
            });

            complete_additive = new_additive.clone();

            current_token = self.peek(0).expect("Token is None");
        }

        complete_additive
    }

    fn parse_term(&mut self) -> Option<ExprNode> {
        // Parser for factor
        let factor = self.parse_factor();

        // Placeholder, en finere måte å gjøre det på? Nei
        let mut complete_factor = factor.clone();

        // Skal peke på neste token i stream
        let mut current_token = self.peek(0).expect("Token is None");
        while current_token.token_type == TokenType::Mul
            || current_token.token_type == TokenType::Div
        {
            let operator = current_token.token_type.clone();
            self.consume(); // Spiser enten * || /
            let next_factor = self.parse_factor();

            let new_factor = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(complete_factor.clone().expect("Incomplete term")),
                    Box::new(next_factor.expect("Next term failed")),
                ),
            });
            complete_factor = new_factor.clone();

            current_token = self.peek(0).expect("Token is None");
        }

        complete_factor
    }

    fn parse_factor(&mut self) -> Option<ExprNode> {
        // Current tok vi kan matche på
        let current_token = self.peek(0).expect("Token is None");
        match current_token.token_type {
            // "(" <expr> ")" case
            TokenType::LParen => {
                self.consume(); // Consume '(' token

                let expression = self.parse_expression();

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
            TokenType::Identifier => self.parse_identifier(),
            _ => {
                println!("Expected factor, found {:?}", current_token.token_type);
                None
            }
        }
    }

    fn parse_identifier(&mut self) -> Option<ExprNode> {
        if let Err(error) = self.expect(TokenType::Identifier) {
            println!("Error {}", error);
            return None;
        }

        let identifier = self.peek(0).expect("Token is None").value.clone();

        self.consume(); // Consume identifier
        Some(ExprNode {
            expr: Expr::Identifier(identifier.unwrap()),
        })
    }

    fn parse_add(&mut self) -> Option<ExprNode> {
        let term = self.parse_term();

        // Placeholder
        let mut complete_term = term.clone();

        // Skal peke på første token i en expr
        let mut current_token = self.peek(0).expect("Token is None");
        // Another term while this is true
        while current_token.token_type == TokenType::Plus
            || current_token.token_type == TokenType::Minus
        {
            let operator = current_token.token_type.clone();
            // Advance stream
            self.consume();
            let next_term = self.parse_term();

            let new_term = Some(ExprNode {
                expr: Expr::BinaryOp(
                    operator,
                    Box::new(complete_term.clone().expect("Incomplete term")),
                    Box::new(next_term.expect("Next term failed")),
                ),
            });
            complete_term = new_term.clone();
            current_token = self.peek(0).expect("Token is None");
        }

        complete_term
    }

    fn parse_unary_operation(&mut self) -> Option<ExprNode> {
        let current_token = self.peek(0).expect("Token is None");

        match current_token.token_type {
            TokenType::BitComplement | TokenType::Minus | TokenType::Not => {
                // Current op: ~, - || !
                let operator = current_token.clone();

                // Consume operator
                println!(
                    "parse_unary_operation Advancing stream, index: {}",
                    self.token_index
                );
                self.consume();

                // Want to parse the expression recursively
                let operand = self.parse_factor();

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

    fn parse_assignment(&mut self) -> Option<StatementNode> {
        // Find variable datatype
        let keyword_type = match self.peek(0).expect("Token is None") {
            current_token if current_token.token_type == TokenType::IntKeyword => {
                current_token.clone()
            }
            current_token if current_token.token_type == TokenType::CharKeyword => {
                current_token.clone()
            }
            _ => return None,
        };
        self.consume(); // Consumes int | char

        // TODO: Kanskje refaktorere for å bruke parse_identifier?
        let identifier_name = match self.peek(0).expect("Token is None") {
            current_token if current_token.token_type == TokenType::Identifier => {
                current_token.clone()
            }
            _ => return None,
        };

        self.consume(); // Consumes identifier

        // False if statement is only a declaration
        let operator: Option<Token>;
        if self.peek(0).unwrap().token_type == TokenType::Assign {
            operator = Some(self.peek(0).unwrap().clone());

            self.consume(); // Consumes '='

        // Assignment is declaration only, do not parse for expression
        } else {
            operator = None;

            if let Err(error) = self.expect(TokenType::Semi) {
                println!("Error {}", error);
                return None;
            }
            // This will consume the ';' before returning keeping the stream correct
            self.consume();
            return Some(StatementNode {
                statement: Statement::Assignment(
                    keyword_type.token_type,
                    identifier_name,
                    operator.map(|op| op.token_type),
                    // Expression is None
                    None,
                ),
            });
        }

        // Parse for the assigned expression
        let assigned_expression = self.parse_expression();

        if let Err(error) = self.expect(TokenType::Semi) {
            println!("Error {}", error);
            return None;
        }

        self.consume(); // Consumes ';'
        Some(StatementNode {
            statement: Statement::Assignment(
                keyword_type.token_type,
                identifier_name,
                operator.map(|op| op.token_type),
                assigned_expression.map(Box::new),
            ),
        })
    }

    fn parse_character(&mut self) -> Option<ExprNode> {
        if let Err(error) = self.expect(TokenType::Char) {
            println!("Error: {}", error);
            return None;
        }

        let char_value = match self.peek(0) {
            Some(current_token) if current_token.token_type == TokenType::Char => {
                current_token.clone()
            }
            _ => {
                println!("Error: Expected a character token.");
                return None;
            }
        };

        let parsed_char = match &char_value.value {
            Some(value) if value.len() == 3 && value.starts_with('\'') && value.ends_with('\'') => {
                value.chars().nth(1) // Henter ut tegnet mellom apostrofene
            }
            _ => {
                println!("Error: Invalid character literal format.");
                return None;
            }
        };

        let parsed_char = match parsed_char {
            Some(c) => c,
            None => {
                println!("Error: No character found in token.");
                return None;
            }
        };

        self.consume();
        // Gjør om char til ascii før den sendes til kode-generering, lar oss bruke Number som vanlig. Men ikke alltid ønskelig?
        Some(ExprNode {
            expr: Expr::Number(parsed_char as i32),
        })
    }

    fn parse_return(&mut self) -> Option<StatementNode> {
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

    fn parse_statement(&mut self) -> Option<StatementNode> {
        if self.token_index > self.token_stream.len() {
            return None;
        }

        let current_token = self.peek(0).expect("Token is None");
        match current_token.token_type {
            TokenType::IntKeyword => self.parse_assignment(),
            TokenType::CharKeyword => self.parse_assignment(),
            TokenType::ReturnKeyword => self.parse_return(),
            TokenType::Identifier => self.parse_decl_assign_for_statement(),
            TokenType::IntLit => self.parse_decl_assign_for_statement(),
            _ => None,
        }
    }

    fn parse_decl_assign_for_statement(&mut self) -> Option<StatementNode> {
        let expression = self.parse_decl_assign();
        if let Err(error) = self.expect(TokenType::Semi) {
            println!("Error {}", error);
            return None;
        }
        self.consume(); // Consume ';'

        Some(StatementNode {
            statement: Statement::DeclAssignForStmnt(expression.map(Box::new)),
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

        while let Some(statement) = self.parse_statement() {
            statement_list.push(statement);

            if let Some(token_type) = self
                .token_stream
                .get(self.token_index)
                .map(|t| t.token_type.clone())
            {
                if token_type == TokenType::RBrace {
                    break;
                }
            }
        }

        if let Err(error) = self.expect(TokenType::RBrace) {
            println!("Error {}", error);
            return None;
        } // }

        self.consume(); // Consume }

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
        let mut function_identifiers: Vec<String> = vec![];

        while let Some(function) = self.parse_function() {
            function_identifiers.push(function.name.clone());
            function_list.push(function);
        }

        Some(ProgramNode {
            body: function_list,
            function_names: function_identifiers,
        })
    }
}
