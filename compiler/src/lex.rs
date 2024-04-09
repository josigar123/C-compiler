use hashbrown::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub enum TokenType {
    IntLit,
    Punctuators,
    Keyword,
    Identifier,
    Operator,
}

pub struct Token {
    pub value: Option<String>,
    pub token_type: TokenType,
}

static PUNCT: [&'static str; 10] = ["{", "}", "(", ")", "[", "]", ":", ";", ",", "."];
static OP: [&'static str; 12] = [
    "+", "-", "*", "/", "=", "==", "!=", "<", "<=", ">", ">=", "!",
];
pub fn get_lexemes(source_file: &str) -> Vec<String> {
    let mut lexemes: Vec<String> = Vec::new();

    let file = File::open(source_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let chars: Vec<char> = line.chars().collect();
        let mut buffer = String::new();

        let mut index = 0;
        while index < chars.len() {
            let ch = chars[index];
            if ch.is_ascii_alphabetic() || ch == '_' {
                buffer.push(ch);
                index += 1;
                while index < chars.len()
                    && (chars[index].is_ascii_alphanumeric() || chars[index] == '_')
                {
                    buffer.push(chars[index]);
                    index += 1;
                }
                lexemes.push(buffer.clone());
                buffer.clear();
            } else if ch.is_ascii_digit() {
                buffer.push(ch);
                index += 1;
                while index < chars.len() && chars[index].is_ascii_digit() {
                    buffer.push(chars[index]);
                    index += 1;
                }
                lexemes.push(buffer.clone());
                buffer.clear();
            } else if PUNCT.contains(&ch.to_string().as_str()) {
                buffer.push(ch);
                index += 1;

                lexemes.push(buffer.clone());
                buffer.clear();
            } else if OP.contains(&ch.to_string().as_str()) {
                let peek = chars[index + 1];
                if (ch == '<' || ch == '>' || ch == '!' || ch == '=') && (peek == '=') {
                    buffer.push(chars[index]);
                    index += 1;
                    buffer.push(chars[index]);
                    index += 1;
                } else {
                    buffer.push(chars[index]);
                    index += 1;
                }

                lexemes.push(buffer.clone());
                buffer.clear();
            } else {
                index += 1;
            }
        }
    }

    println!("From get_lexemes");
    for lex in &lexemes {
        println!("Lexeme({}) ", lex);
    }

    lexemes
}

// Start 15:30, slutt 16:40, tir 26. mars
// TODO: Noe rart med Operator regex, eller ser alt ut til å funke, operator regex er det eneste som ikke blir gjenkjent
// DONE: 23:52, ChatGPT ga feil regex tidligere (nå kommentert ut), escapa feil
// Den nye er også generert av GPT, men fungerer nå som forventet
static INTEGER_LITERAL: &str = r"^(0|[1-9]\d*)$";
static KEYWORDS: &str = r#"\b(int|char|return|if|while|else|for|else if|struct|enum)\b"#;
static IDENTIFIERS: &str = r#"\b[a-zA-Z_][a-zA-Z0-9_]*\b"#;
static OPERATORS: &str = r"[\+\-\*/%&|^=!<>]=?|>>=?|<<=?|&&|\|\||\+\+|--|->|==|!=|<=|>=|\?";
static PUNCTUATORS: &str = r#"[{}()\[\]:;,.]"#;

lazy_static! {
    static ref KEYWORD_HASHSET: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("int");
        set.insert("char");
        set.insert("return");
        set.insert("if");
        set.insert("while");
        set.insert("else");
        set.insert("for");
        set.insert("else if");
        set.insert("struct");
        set.insert("enum");
        set.insert("static");
        set.insert("break");
        set.insert("continue");
        set.insert("void");
        set.insert("union");
        set
    };
}

pub fn tokenize_lexemes(lexemes: Vec<String>) -> Vec<Token> {
    println!("From tokenize_lexemes");
    let mut tokens: Vec<Token> = Vec::new();

    let integer_literal_regex = Regex::new(INTEGER_LITERAL).unwrap();
    let identifiers_regex = Regex::new(IDENTIFIERS).unwrap();
    let punctuators_regex = Regex::new(PUNCTUATORS).unwrap();
    let keywords_regex = Regex::new(KEYWORDS).unwrap();
    let operators_regex = Regex::new(OPERATORS).unwrap();

    for lexeme in lexemes {
        if keywords_regex.is_match(&lexeme) {
            println!("From keywords case, Lexeme to tokenize: {}", lexeme);
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::Keyword,
            });
        } else if integer_literal_regex.is_match(&lexeme) {
            println!("From int_lit case, Lexeme to tokenize: {}", lexeme);
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::IntLit,
            });
        } else if identifiers_regex.is_match(&lexeme) {
            println!("From identifier case, Lexeme to tokenize: {}", lexeme);
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::Identifier,
            });
        } else if punctuators_regex.is_match(&lexeme) {
            println!("From punctuator case, Lexeme to tokenize: {}", lexeme);
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::Punctuators,
            });
        } else if operators_regex.is_match(&lexeme) {
            println!("From operator case, Lexeme to tokenize: {}", lexeme);
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::Operator,
            });
        } else {
            println!("Unrecognized {}", lexeme);
        }
    }

    return tokens;
}
