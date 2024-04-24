use std::fs;
use std::vec;
mod gen;
mod lex;
mod parse_util;
mod parser;
mod token;

// Loggfør ish 2 timer 30 min for lørdag 20.april 16:00 -18:30
// Loggfør 30min ish for søndat 21.april 15:30-16:00, merk denne dag! I dag har vi en fungerende
// kompilator for verdens enkleste program!

// start 16:00 onsdag 24 april, ferdig med 3 unary ops 18:20, kan relativt greit legge inn flere
fn main() {
    let mut test_tokens: Vec<token::Token> = vec![];

    test_tokens = test_lexer("tests/lexer_tests/new_token_test.c");
    print_tokens(test_tokens);

    compile();
}

pub fn test_lexer(lex_this: &str) -> Vec<token::Token> {
    let mut lexemes: Vec<String> = vec![];
    let mut tokens: Vec<token::Token> = vec![];

    lexemes = lex::get_lexemes(lex_this);
    tokens = lex::tokenize_lexemes(lexemes);

    tokens
}

pub fn print_tokens(tokens: Vec<token::Token>) {
    for token in &tokens {
        println!("Value: {:?}\nType: {:?}", token.value, token.token_type);
        println!();
    }
}

pub fn compile() {
    // Lexing
    let return_int = "tests/parser_tests/return_int.c";
    let output_assembly_path = "bin/out.s";

    let mut lexemes: Vec<String> = vec![];
    let mut tokens: Vec<token::Token> = vec![];

    lexemes = lex::get_lexemes(return_int);
    tokens = lex::tokenize_lexemes(lexemes);

    let mut parser = parser::Parser::new(tokens);

    let program_node = parser.parse_program().expect("Failed to parse program");
    //program_node.walk_and_print();

    let generator = gen::Generator::new(program_node);

    let asm = generator.walk_da_tree();
    println!();

    match fs::write(output_assembly_path, asm) {
        Ok(_) => println!("File generated at {}", output_assembly_path),
        Err(e) => eprintln!("Error writing to {}: {}", output_assembly_path, e),
    }
}
