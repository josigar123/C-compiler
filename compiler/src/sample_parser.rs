// A sample top-down recursive descent parser for parsing a simple return statement
// e.g. return 3;
// En T-D RDC skal ha en rekursiv funksjon for hver NON_TERMINAL og ta høyde for dens produksjonsregler

/*
   Expected tokens:
       return: Keyword
       3     : IntLit

*/

use crate::lex;
use lex::Token;

pub enum ASTNode {
    Expr(i32),
    Return(Box<ASTNode>),
    FuncDecl(String, Box<ASTNode>),
    Prog(Box<ASTNode>),
}

pub fn parse_statement(tokens: &Vec<Token>, token_index: usize) -> ASTNode {
    println!("From parse_statement");
    let mut next_token = &tokens[token_index];

    if next_token.value != Some("return".to_string()) {
        panic!();
    }
    println!("Return parsed!");
    next_token = &tokens[token_index + 1];

    if next_token.value != Some("3".to_string()) {
        panic!();
    }
    println!("3 parsed!");
    next_token = &tokens[token_index + 2];

    if next_token.value != Some(";".to_string()) {
        panic!();
    }
    println!("; parsed!");
    let statement_node = ASTNode::Return(Box::new(ASTNode::Expr(3)));
    return statement_node;
}
