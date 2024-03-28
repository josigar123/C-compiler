use regex::Regex;

mod lex;

fn main() {
    // No unrecognized output = good
    let file_to_tokenize = "tests/lexer_tests/file_to_tokenize.c";

    // No unrecognized output = good
    let recognize_test = "tests/lexer_tests/recognize_test.txt";

    // No output = good
    let unrecognize_test = "tests/lexer_tests/unrecognized_test.txt";
    let mut lexemes: Vec<String> = Vec::new();
    lexemes = lex::get_lexemes(recognize_test);

    let mut tokens: Vec<lex::Token> = Vec::new();
    tokens = lex::tokenize_lexemes(lexemes);
}
