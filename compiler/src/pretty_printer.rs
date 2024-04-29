use crate::parser::{Expr, ExprNode, FunctionNode, ProgramNode, Statement, StatementNode};
use crate::token::TokenType;
use std::fmt::{self};

impl fmt::Display for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function_node in &self.body {
            write!(f, "{}", function_node)?;
        }
        Ok(())
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::StringLit => write!(f, "string literal"),
            TokenType::IntLit => write!(f, "integer literal"),
            TokenType::Identifier => write!(f, "identifier"),
            TokenType::Underscore => write!(f, "_"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrack => write!(f, "["),
            TokenType::RBrack => write!(f, "]"),
            TokenType::Semi => write!(f, ";"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Colon => write!(f, ":"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Div => write!(f, "/"),
            TokenType::Mul => write!(f, "*"),
            TokenType::Assign => write!(f, "="),
            TokenType::Not => write!(f, "!"),
            TokenType::And => write!(f, "&&"),
            TokenType::Or => write!(f, "||"),
            TokenType::BitAnd => write!(f, "&"),
            TokenType::BitOr => write!(f, "|"),
            TokenType::Xor => write!(f, "^"),
            TokenType::Mod => write!(f, "%"),
            TokenType::BitComplement => write!(f, "~"),
            TokenType::Eq => write!(f, "=="),
            TokenType::Lt => write!(f, "<"),
            TokenType::Gt => write!(f, ">"),
            TokenType::Le => write!(f, "<="),
            TokenType::Ge => write!(f, ">="),
            TokenType::Neq => write!(f, "!="),
            TokenType::ReturnKeyword => write!(f, "return"),
            TokenType::IfKeyword => write!(f, "if"),
            TokenType::ElseKeyword => write!(f, "else"),
            TokenType::ElseIfKeyword => write!(f, "else if"),
            TokenType::EnumKeyword => write!(f, "enum"),
            TokenType::WhileKeyword => write!(f, "while"),
            TokenType::ForKeyword => write!(f, "for"),
            TokenType::IntKeyword => write!(f, "int"),
            TokenType::CharKeyword => write!(f, "char"),
            TokenType::StructKeyword => write!(f, "struct"),
            TokenType::UnionKeyword => write!(f, "union"),
            TokenType::ContinueKeyword => write!(f, "continue"),
            TokenType::BreakKeyword => write!(f, "break"),
            TokenType::StaticKeyword => write!(f, "static"),
            TokenType::VoidKeyword => write!(f, "void"),
            TokenType::Error => write!(f, "error"),
        }
    }
}

impl fmt::Display for FunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "fn {}() -> {} {{\n", self.name, self.return_value)?;
        for statement_node in &self.body {
            write!(f, "{}", statement_node)?;
        }
        write!(f, "}}\n\n")
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Return(expr_node) => write!(f, "return {};", expr_node),
        }
    }
}

impl fmt::Display for StatementNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t{}\n", self.statement)
    }
}

impl fmt::Display for ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{}", num),
            Expr::UnaryOp(operator, expr) => {
                if let Some(expr) = expr {
                    write!(f, "({:?} {})", operator, expr)
                } else {
                    write!(f, "({:?})", operator)
                }
            }
            Expr::BinaryOp(operator, left, right) => {
                write!(f, "({} {:?} {})", left, operator, right)
            }
        }
    }
}
