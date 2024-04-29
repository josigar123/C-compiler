use std::fs;
use std::vec;
mod gen;
mod lex;
mod parse_util;
mod parser;
mod pretty_printer;
mod token;

// LOGGFØR start 17:20 - 18:00, fikset unop parsing, søndag 28 april
// Enkel fiks parset for en følgene fakto
fn main() {
    //let mut test_tokens: Vec<token::Token> = vec![];

    //test_tokens = test_lexer("tests/lexer_tests/new_token_test.c");
    //print_tokens(test_tokens);

    let return_int = "tests/parser_tests/return_int.c";
    let _output_assembly_path = "bin/out.s";

    let mut _lexemes: Vec<String> = vec![];
    let mut _tokens: Vec<token::Token> = vec![];

    _lexemes = lex::get_lexemes(return_int);
    _tokens = lex::tokenize_lexemes(_lexemes);

    let mut parser = parser::Parser::new(_tokens);

    let program_node = parser.parse_program().expect("Failed to parse program");
    println!("{}", program_node);
    compile();
}

pub fn test_lexer(lex_this: &str) -> Vec<token::Token> {
    let mut _lexemes: Vec<String> = vec![];
    let mut _tokens: Vec<token::Token> = vec![];

    _lexemes = lex::get_lexemes(lex_this);
    _tokens = lex::tokenize_lexemes(_lexemes);

    _tokens
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

    let mut _lexemes: Vec<String> = vec![];
    let mut _tokens: Vec<token::Token> = vec![];

    _lexemes = lex::get_lexemes(return_int);
    _tokens = lex::tokenize_lexemes(_lexemes);

    // Parsing
    let mut parser = parser::Parser::new(_tokens);

    let program_node = parser.parse_program().expect("Failed to parse program");
    println!("{}", program_node);

    // Generating
    let generator = gen::Generator::new(program_node);

    let asm = generator.walk_da_tree();
    println!();

    match fs::write(output_assembly_path, asm) {
        Ok(_) => println!("File generated at {}", output_assembly_path),
        Err(e) => eprintln!("Error writing to {}: {}", output_assembly_path, e),
    }
}
