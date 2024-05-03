use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

// Many of Cs tokens, String literals are not yet supported by the lexer
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Literals
    StringLit,
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
    ElseIfKeyword,
    EnumKeyword,
    WhileKeyword,
    ForKeyword,
    IntKeyword,
    CharKeyword,
    StructKeyword,
    UnionKeyword,
    ContinueKeyword,
    BreakKeyword,
    StaticKeyword,
    VoidKeyword,
    // Error Token
    Error,
}

// Represents a Token
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: Option<String>,
    pub token_type: TokenType,
}

// REGEX
lazy_static! {
    pub static ref INTEGER_LITERAL: Regex = Regex::new(r"^(0|[1-9]\d*)$").unwrap();
    pub static ref IDENTIFIERS: Regex = Regex::new(r#"\b[a-zA-Z_][a-zA-Z0-9_]*\b"#).unwrap();
    pub static ref CHAR_LITERAL: Regex = Regex::new(r"'[^']'").unwrap();
}

// MAPS
lazy_static! {
    pub static ref SPECIAL_CHARACTER_MAP: HashMap<char, TokenType> = {
        let mut map = HashMap::new();
        map.insert('_', TokenType::Underscore);
        map
    };
}

lazy_static! {
    pub static ref KEYWORD_MAP: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("int", TokenType::IntKeyword);
        map.insert("char", TokenType::CharKeyword);
        map.insert("return", TokenType::ReturnKeyword);
        map.insert("if", TokenType::IfKeyword);
        map.insert("while", TokenType::WhileKeyword);
        map.insert("else", TokenType::ElseKeyword);
        map.insert("for", TokenType::ForKeyword);
        map.insert("else if", TokenType::ElseIfKeyword);
        map.insert("struct", TokenType::StructKeyword);
        map.insert("enum", TokenType::EnumKeyword);
        map.insert("static", TokenType::StaticKeyword);
        map.insert("break", TokenType::BreakKeyword);
        map.insert("continue", TokenType::ContinueKeyword);
        map.insert("void", TokenType::VoidKeyword);
        map.insert("union", TokenType::UnionKeyword);
        map
    };
}

lazy_static! {
    pub static ref OPERATOR_MAP: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("+", TokenType::Plus);
        map.insert("=", TokenType::Assign);
        map.insert("%", TokenType::Mod);
        map.insert("-", TokenType::Minus);
        map.insert("*", TokenType::Mul);
        map.insert("/", TokenType::Div);
        map.insert("||", TokenType::Or);
        map.insert("&&", TokenType::And);
        map.insert("|", TokenType::BitOr);
        map.insert("&", TokenType::BitAnd);
        map.insert("^", TokenType::Xor);
        map.insert("!", TokenType::Not);
        map.insert("~", TokenType::BitComplement);
        map
    };
}

lazy_static! {
    pub static ref PUNCTUATOR_MAP: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("{", TokenType::LBrace);
        map.insert("}", TokenType::RBrace);
        map.insert("(", TokenType::LParen);
        map.insert(")", TokenType::RParen);
        map.insert("[", TokenType::LBrack);
        map.insert("]", TokenType::RBrack);
        map.insert(".", TokenType::Dot);
        map.insert(",", TokenType::Comma);
        map.insert(";", TokenType::Semi);
        map.insert(":", TokenType::Colon);
        map
    };
}

lazy_static! {
    pub static ref COMPARATOR_MAP: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("==", TokenType::Eq);
        map.insert("<", TokenType::Lt);
        map.insert(">", TokenType::Gt);
        map.insert("<=", TokenType::Le);
        map.insert(">=", TokenType::Ge);
        map.insert("!=", TokenType:: Neq);
        // A small union with the operators, needed for the != check
        map.insert("!", TokenType::Not);
        map
    };
}
