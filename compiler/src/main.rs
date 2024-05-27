use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec;
mod gen;
mod lex;
mod parse_util;
mod parser;
mod pretty_printer;
mod symbol_table;
mod token;

fn main() {
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

/*
pub fn compile() {
    // Lexing
    let return_int = "tests/parser_tests/return_int.c";
    let output_assembly_path = "bin/out.s";
    let mut _lexemes: Vec<String> = vec![];
    let mut _tokens: Vec<token::Token> = vec![];

    _lexemes = lex::get_lexemes(return_int);
    _tokens = lex::tokenize_lexemes(_lexemes);

    let symbol_table_thread_unsdafe = symbol_table::SymbolTable::new();
    let symbol_table_thread_safe = Arc::new(Mutex::new(symbol_table_thread_unsdafe));
    // Parsing
    let mut parser = parser::Parser::new(_tokens, symbol_table_thread_safe);

    let program_node = parser.parse_program().expect("Failed to parse program");
    println!("{}", program_node);

    let symbol_table_clone = symbol_table_thread_safe.clone();
    let symbol_table = symbol_table_clone.lock().unwrap();
    symbol_table.pretty_print();

    // Generating
    let generator = gen::Generator::new(program_node);

    let asm = generator.walk_da_tree();
    println!();

    match fs::write(output_assembly_path, asm) {
        Ok(_) => println!("File generated at {}", output_assembly_path),
        Err(e) => eprintln!("Error writing to {}: {}", output_assembly_path, e),
    }
}
*/

pub fn compile() {
    // Lexing
    let return_int = "tests/parser_tests/return_int.c";
    let output_assembly_path = "bin/out.s";
    let mut _lexemes: Vec<String> = vec![];
    let mut _tokens: Vec<token::Token> = vec![];

    _lexemes = lex::get_lexemes(return_int);
    _tokens = lex::tokenize_lexemes(_lexemes);

    // Create a symbol table (thread safe)
    let symbol_table_thread_unsafe = symbol_table::SymbolTable::new();
    let symbol_table_thread_safe = Arc::new(Mutex::new(symbol_table_thread_unsafe));

    // Parsing
    let cloned_symbol_table = symbol_table_thread_safe.clone(); // Clone the Arc
    let mut parser = parser::Parser::new(_tokens.clone(), cloned_symbol_table);

    let program_node = parser.parse_program().expect("Failed to parse program");
    println!("{}", program_node);

    // Print the symbol table
    let symbol_table = symbol_table_thread_safe.lock().unwrap();
    symbol_table.pretty_print();

    // Generating
    let generator = gen::Generator::new(program_node,symbol_table_thread_safe.clone());

    let asm = generator.walk_da_tree();
    println!();

    // Write assembly code to file
    match fs::write(output_assembly_path, asm) {
        Ok(_) => println!("File generated at {}", output_assembly_path),
        Err(e) => eprintln!("Error writing to {}: {}", output_assembly_path, e),
    }
}
