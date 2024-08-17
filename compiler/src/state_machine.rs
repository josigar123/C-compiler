use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
pub enum State{
    Identifier,
    Keyword,
    Punctuator,
    Operator,
    Default,
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenClass {
    // Literals
    IntLit,
    Char,
    // Identifier
    Identifier,
    // Special Characters
    Underscore,
    // Punctuators
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBrack,
    RBrack,
    Semi,
    Comma,
    Dot,
    Colon,
    // Operators
    Plus,   // +
    Minus,  // -
    Div,    // /
    Mul,    // *
    Assign, // =
    Not,    // !
    And,    // &&
    Or,     // ||
    BitAnd, // &
    BitOr,  // |
    Xor,    // ^
    Mod,    // %
    BitComplement,
    // Comparators
    Eq,  // ==
    Lt,  // <
    Gt,  // >
    Le,  // <=
    Ge,  // >=
    Neq, // !=
    // Keywords
    ReturnKeyword,
    IfKeyword,
    ElseKeyword,
    IntKeyword,
    CharKeyword,
    // Error Token
    Error,
}

pub struct Token<'a> {
    class: TokenClass,
    value: &'a str,
}

pub struct Lexer<'a> {
    file_path: &'a str,
    current_state: State,
    tokens: Vec<Token<'a>>,
}

impl Lexer {

    pub fn new(file_path: &str) -> Self {Lexer{file_path, current_state: State::Default, tokens: vec![]}}

    pub fn retrieve_lexemes(&self) -> Vec<&str>{
        let mut lexemes: Vec<&str> = vec![];
        let file_path: &str = self.file_path;

        let reader = Self::get_buf_reader(file_path);
        for line in reader.lines() {
            let line = line.expect("In state_machine.rs 'retrieve_lexemes' Failed to read line");
            let chars = line.chars().collect();
            let mut buffer = String::new();

        }

        lexemes
    }

    fn get_buf_reader(file_path: &str) -> BufReader<File> {
        let file = File::open(file_path).expect("In state_machine.rs 'get_buf_reader': failed to open file");
        BufReader::new(file)
    }

}