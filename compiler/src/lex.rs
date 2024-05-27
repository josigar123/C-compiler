use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::token::{Token, TokenType};

// MAPS
use crate::token::{
    COMPARATOR_MAP, KEYWORD_MAP, OPERATOR_MAP, PUNCTUATOR_MAP, SPECIAL_CHARACTER_MAP,
};

// REGEX
use crate::token::{CHAR_LITERAL, IDENTIFIERS, INTEGER_LITERAL};

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
            if ch.is_ascii_alphabetic() || SPECIAL_CHARACTER_MAP.contains_key(&ch) {
                buffer.push(ch);
                index += 1;
                while index < chars.len()
                    && (chars[index].is_ascii_alphanumeric()
                        || SPECIAL_CHARACTER_MAP.contains_key(&chars[index]))
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
            } else if PUNCTUATOR_MAP.contains_key(&ch.to_string().as_str()) {
                buffer.push(ch);
                index += 1;

                lexemes.push(buffer.clone());
                buffer.clear();
            } else if OPERATOR_MAP.contains_key(&ch.to_string().as_str()) {
                let peek = chars.get(index + 1);

                // Case when its comparison
                if peek == Some(&'=') && ch == '=' {
                    buffer.push(ch);
                    buffer.push(*peek.unwrap());
                    index += 2;
                } else {
                    match (ch, peek) {
                        ('|', Some(&'|')) | ('&', Some(&'&')) | ('!', Some(&'=')) => {
                            buffer.push(ch);
                            buffer.push(*peek.unwrap());
                            index += 2;
                        }
                        (_, _) => {
                            buffer.push(ch);
                            index += 1;
                        }
                    }
                }
                lexemes.push(buffer.clone());
                buffer.clear();
            } else if COMPARATOR_MAP.contains_key(&ch.to_string().as_str()) {
                let peek = chars[index + 1];
                if (ch == '<' || ch == '>' || ch == '!') && (peek == '=') {
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
            } else if ch == '\'' {
                buffer.push(ch);
                index += 1;
                if chars[index] != '\'' {
                    buffer.push(chars[index]);
                    index += 1;
                    if chars[index] == '\'' {
                        buffer.push(chars[index]);
                        index += 1;
                    }
                }

                lexemes.push(buffer.clone());
                buffer.clear();
            } else {
                index += 1;
            }
        }
    }

    lexemes
}

pub fn tokenize_lexemes(lexemes: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let integer_literal_regex = &INTEGER_LITERAL;
    let identifiers_regex = &IDENTIFIERS;
    let char_regex = &CHAR_LITERAL;

    for lexeme in lexemes {
        if integer_literal_regex.is_match(&lexeme) {
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::IntLit,
            });
        } else if identifiers_regex.is_match(&lexeme) {
            // If lexeme has value in map
            if let Some(token_type) = KEYWORD_MAP.get(lexeme.as_str()) {
                tokens.push(Token {
                    value: Some(lexeme.to_string()),
                    token_type: (*token_type).clone(),
                });
            } else {
                tokens.push(Token {
                    value: Some(lexeme.to_string()),
                    token_type: TokenType::Identifier,
                });
            }
        } else if char_regex.is_match(&lexeme) {
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::Char,
            });
        } else if PUNCTUATOR_MAP.contains_key(lexeme.as_str()) {
            if let Some(token_type) = PUNCTUATOR_MAP.get(lexeme.as_str()) {
                tokens.push(Token {
                    value: Some(lexeme.to_string()),
                    token_type: (*token_type).clone(),
                });
            }
        } else if OPERATOR_MAP.contains_key(lexeme.as_str()) {
            if let Some(token_type) = OPERATOR_MAP.get(lexeme.as_str()) {
                tokens.push(Token {
                    value: Some(lexeme.to_string()),
                    token_type: (*token_type).clone(),
                });
            }
        } else if COMPARATOR_MAP.contains_key(lexeme.as_str()) {
            if let Some(token_type) = COMPARATOR_MAP.get(lexeme.as_str()) {
                tokens.push(Token {
                    value: Some(lexeme.to_string()),
                    token_type: (*token_type).clone(),
                });
            }
        } else {
            println!("Unrecognized {}", lexeme);
            tokens.push(Token {
                value: Some(lexeme.to_string()),
                token_type: TokenType::Error,
            });
        }
    }

    tokens
}
