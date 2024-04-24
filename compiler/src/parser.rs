use crate::token::{self, Token, TokenType};

pub enum Expr {
    Number(i32),
}

pub struct ExprNode {
    pub expr: Expr,
}

//REFACTOR: Split into AssignmentStruct for å øke lesbarhet
// Og en eventuell enum for å holde de forskjellige verdiene som kan tilegnes Ident
pub enum Statement {
    Return(ExprNode),
    // Assignment(AssignmentConstruct),
}

pub struct AssignmentConstruct {
    data_type: TokenType,
    var_name: TokenType,
    assignment: TokenType,
    var_val: String,
    semi: TokenType,
}

pub struct StatementNode {
    pub statement: Statement,
}

pub struct FunctionNode {
    pub return_value: TokenType,
    pub name: String, // Kan evt være expected tokentype Ident
    pub body: Vec<StatementNode>,
}

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

    // Consume a token from token stream, øker index "Konsumerer"
    fn consume(&mut self) {
        self.token_index += 1;
    }

    // Peek en token, returnerer den, advancer ikke token_stream
    fn peek(&mut self, offset: usize) -> Option<&Token> {
        if self.token_index + offset >= self.token_stream.len() {
            return None;
        }
        return self.token_stream.get(self.token_index + offset);
    }

    // Forvent token, e.g ved funksjoner forventes en struktur
    fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        // Checks the current token
        match self.peek(0) {
            Some(token) if token.token_type == expected => Ok(()),
            Some(token) => Err(format!("Expected {:?} but found {:?}", expected, token)),
            None => Err("Unexpected stream end".to_string()),
        }
    }

    // Expr er nå kun i32
    fn parse_expression(&mut self) -> Option<ExprNode> {
        // Bekrefter at token vil ha en verdi
        if self.token_index >= self.token_stream.len() {
            return None;
        }

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

// Fra ChatGPT

impl ProgramNode {
    pub fn walk_and_print(&self) {
        for function in &self.body {
            function.walk_and_print(0);
        }
    }
}

impl FunctionNode {
    fn walk_and_print(&self, indent_level: usize) {
        println!(
            "{}Function: {} -> {:?}",
            " ".repeat(indent_level * 4),
            self.name,
            self.return_value
        );
        for statement in &self.body {
            statement.walk_and_print(indent_level + 1);
        }
    }
}

impl StatementNode {
    fn walk_and_print(&self, indent_level: usize) {
        match &self.statement {
            Statement::Return(expr_node) => {
                println!("{}Return Statement", " ".repeat(indent_level * 4));
                expr_node.walk_and_print(indent_level + 1);
            }
        }
    }
}

impl ExprNode {
    fn walk_and_print(&self, indent_level: usize) {
        match &self.expr {
            Expr::Number(num) => {
                println!("{}Number: {}", " ".repeat(indent_level * 4), num);
            }
        }
    }
}
