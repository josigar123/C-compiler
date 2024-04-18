use crate::token;
use crate::token::{Token, TokenType};

// TODO: FIKS Function name, lag og parse_program og en hoved parse funksjon som skal returnere roten til treet.
// Prøv å generer kode når dette er gjort
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
    pub name: String, // Kan evt være expected tokentype Ident
    pub body: Vec<StatementNode>,
}

pub struct ProgramNode {
    pub body: Vec<FunctionNode>,
}

pub struct Parser {
    token_index: usize,
    token_stream: Vec<Token>,
}

impl Parser {
    // Constructor, implicitly sets index to 0
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_index: 0,
            token_stream: tokens,
        }
    }

    // Expr er nå kun i32
    fn parse_expression(&mut self) -> Option<ExprNode> {
        // Bekrefter at token vil ha en verdi
        if self.token_index > self.token_stream.len() {
            return None;
        }

        // Forventer at Expr skal være et heltall
        self.expect(TokenType::IntLit);

        // Parser heltall
        let parsed_int: i32 = match self.token_stream[self.token_index]
            .value
            .as_ref()
            .unwrap()
            .parse()
        {
            Ok(parsed) => parsed,
            // Kan returne None for nå, men bør si ifra at heltall er forventet
            Err(_) => return None,
        };
        //Returnerer expression node
        Some(ExprNode {
            expr: Expr::Number(parsed_int),
        })
    }

    fn parse_statement(&mut self) -> Option<StatementNode> {
        if self.token_index > self.token_stream.len() {
            return None;
        }

        // Forventer return da dette er eneste expression
        self.expect(TokenType::ReturnKeyword);

        let statement_expression = self.parse_expression().unwrap();

        // Konsumerer for å stå på neste token
        self.consume();
        // Neste token er forventet å være semikolon
        self.expect(TokenType::Semi);
        self.consume();

        Some(StatementNode {
            statement: Statement::Return(statement_expression),
        })
    }

    /*
       Func har:
           return type: IntKeyword
           name: main
           body: List of StatementNode
    */
    fn parse_function(&mut self) -> Option<FunctionNode> {
        if self.token_index > self.token_stream.len() {
            return None;
        }

        // Holder statements
        let mut statement_list: Vec<StatementNode> = vec![];

        // Expect IntKeyword
        self.expect(TokenType::IntKeyword); // int

        let return_type = self
            .token_stream
            .get(self.token_index)
            .unwrap()
            .token_type
            .clone();

        self.consume();
        self.expect(TokenType::Identifier); // main

        let function_name = self
            .token_stream
            .get(self.token_index)
            .unwrap()
            .value
            .clone()
            .unwrap();
        self.consume();
        self.expect(TokenType::LParen); // (
        self.consume();
        self.expect(TokenType::RParen); // )
        self.consume();
        self.expect(TokenType::LBrace); // {
        self.consume();

        let statement = self.parse_statement();

        self.expect(TokenType::RBrace); // }

        // Kun 1 statement for nå
        statement_list.push(statement.unwrap());
        Some(FunctionNode {
            return_value: return_type,
            name: function_name,
            body: statement_list,
        })
    }

    fn parse_program(&mut self) -> Option<ProgramNode> {
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

    // ############# PARSER UTIL START ####################
    // Consume a token from token stream
    fn consume(&mut self) {
        self.token_index += 1;
    }

    // Peek en token, returnerer den
    fn peek(&mut self, offset: usize) -> Option<&Token> {
        return self.token_stream.get(self.token_index + offset);
    }

    // Få neste token i tokenstream, øker og index med 1
    fn next(&mut self) -> Option<Token> {
        if self.token_index < self.token_stream.len() {
            let token = self.token_stream[self.token_index].clone();
            self.token_index += 1;
            Some(token)
        } else {
            None
        }
    }

    // Forvent token, e.g ved funksjoner forventes en struktur
    fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        match self.next() {
            Some(token) if token.token_type == expected => Ok(()),
            Some(token) => Err(format!("Expected {:?} but found {:?}", expected, token)),
            None => Err("Unexpected stream end".to_string()),
        }
    }
    // ############# PARSER UTIL START ####################
}