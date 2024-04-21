use std::fs;
use std::vec;
mod gen;
mod lex;
mod parser;
mod token;

// Loggfør ish 2 timer 30 min for lørdag 20.april 16:00 -18:30
fn main() {
    // Lexing
    let return_int = "tests/parser_tests/return_int.c";
    let output_assembly_path = "bin/out.s";

    let mut lexemes: Vec<String> = vec![];
    let mut tokens: Vec<token::Token> = vec![];

    lexemes = lex::get_lexemes(return_int);
    tokens = lex::tokenize_lexemes(lexemes);

    let mut parser = parser::Parser::new(tokens);

    let program_node = parser.parse_program().expect("Failed to parse program");
    program_node.walk_and_print();

    let generator = gen::Generator::new(program_node);

    let asm = generator.walk_da_tree();
    println!();

    match fs::write(output_assembly_path, asm) {
        Ok(_) => println!("File generated at {}", output_assembly_path),
        Err(e) => eprintln!("Error writing to {}: {}", output_assembly_path, e),
    }
}
