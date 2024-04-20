use std::vec;

mod gen;
mod lex_refactor;
mod magic_parsing;
mod token;

// Loggfør ish 4 timer for lexer refaktorering søndag 7 april 12-16
fn main() {
    // Lexing
    let return_int = "tests/parser_tests/return_int.c";

    let mut lexemes: Vec<String> = vec![];
    let mut tokens: Vec<token::Token> = vec![];

    lexemes = lex_refactor::get_lexemes(&return_int);
    tokens = lex_refactor::tokenize_lexemes(lexemes);

    let mut parser = magic_parsing::Parser::new(tokens);

    let program_node = parser.parse_program();
    program_node.unwrap().walk_and_print();
    println!("Program successfully parsed!");
}
