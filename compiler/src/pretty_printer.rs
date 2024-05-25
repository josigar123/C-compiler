use crate::parser::{Expr, ExprNode, FunctionNode, ProgramNode, Statement, StatementNode};
use crate::token::{Token, TokenType};
use std::fmt::{self};

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{}({})", self.token_type, value)
        } else {
            write!(f, "{}", self.token_type)
        }
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
            TokenType::Char => write!(f, "char"),
        }
    }
}

impl fmt::Display for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "program {{")?;
        for function_node in &self.body {
            writeln!(f, "{}", function_node)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for FunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  function {}() -> {} {{", self.name, self.return_value)?;
        for statement_node in &self.body {
            writeln!(f, "    {}", statement_node)?;
        }
        writeln!(f, "  }}")?;
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Return(expr_node) => writeln!(f, "return {};", expr_node),
            Statement::Assignment(data_type, name, _, expr_node) => {
                writeln!(
                    f,
                    "{} {} = {};",
                    data_type,
                    name,
                    format_option_expr_node(expr_node)
                )
            }
            Statement::DeclAssignForStmnt(expression) => {
                writeln!(f, "{};", format_option_expr_node(expression))
            }
        }
    }
}

impl fmt::Display for StatementNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.statement)
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
            Expr::Identifier(ident) => write!(f, "{}", ident),
            Expr::UnaryOp(operator, expr) => {
                write!(f, "({} ", operator)?;
                if let Some(expr) = expr {
                    write!(f, "{}", expr)?;
                } else {
                    write!(f, "None")?;
                }
                write!(f, ")")
            }
            Expr::BinaryOp(operator, left, right) => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expr::DeclAssign(ident, assignment, expr) => {
                write!(
                    f,
                    "({} {} {})",
                    format_option_expr_node(ident),
                    format_option_token_type(assignment),
                    format_option_expr_node(expr)
                )
            }
        }
    }
}

// Helper function to format Option<Box<ExprNode>>
fn format_option_expr_node(opt: &Option<Box<ExprNode>>) -> String {
    match opt {
        Some(expr_node) => format!("{}", expr_node),
        None => "None".to_string(),
    }
}

// Helper function to format Option<TokenType>
fn format_option_token_type(opt: &Option<TokenType>) -> String {
    match opt {
        Some(token_type) => format!("{}", token_type),
        None => "None".to_string(),
    }
}
