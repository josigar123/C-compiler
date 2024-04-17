mod lex;
mod lex_refactor;
mod min_magiske_parser;
mod sample_parser;
mod token;

// Loggfør ish 4 timer for lexer refaktorering søndag 7 april 12-16
fn main() {
    // ############## LEXER START ###############
    /*
        // No unrecognized output = good
        let file_to_tokenize = "tests/lexer_tests/file_to_tokenize.c";

        let large_test = "tests/lexer_tests/larger_file.c";

        // No unrecognized output = good
        let recognize_test = "tests/lexer_tests/recognize_test.txt";

        // No output = good
        let unrecognize_test = "tests/lexer_tests/unrecognized_test.txt";
        let mut lexemes: Vec<String> = Vec::new();
        lexemes = lex::get_lexemes(large_test);

        let mut tokens: Vec<lex::Token> = Vec::new();
        tokens = lex::tokenize_lexemes(lexemes);
    */
    // ############## LEXER END #################

    // ############### REFACTORED LEXER START #############

    let large_test = "tests/lexer_tests/larger_file.c";
    let file_to_tokenize = "tests/lexer_tests/file_to_tokenize.c";
    let recognize_test = "tests/lexer_tests/recognize_test.txt";
    let mut lexemes: Vec<String> = Vec::new();
    lexemes = lex_refactor::get_lexemes(large_test);

    let mut tokens: Vec<token::Token> = Vec::new();
    tokens = lex_refactor::tokenize_lexemes(lexemes);

    // ############### REFACTORED LEXER END ###############
    // ############## PARSER START ##############
    /*
    let file_to_parse = "tests/parser_tests/sample_parser_test.txt";
    let mut parse_lexemes: Vec<String> = Vec::new();
    parse_lexemes = lex::get_lexemes(file_to_parse);

    let mut parse_tokens: Vec<lex::Token> = Vec::new();
    parse_tokens = lex::tokenize_lexemes(parse_lexemes);

    sample_parser::parse_statement(&parse_tokens, 0);
    */
    // ########### PARSER END ####################
}
