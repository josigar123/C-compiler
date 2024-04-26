use crate::parser::{Expr, ExprNode, FunctionNode, Parser, ProgramNode, Statement, StatementNode};
use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{}", num),
            Expr::UnaryOp(op, expr) => {
                if let Some(expr) = expr {
                    write!(f, "({:?} {})", op, expr)
                } else {
                    write!(f, "({:?})", op)
                }
            }
        }
    }
}

impl fmt::Display for ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Return(expr_node) => write!(f, "Return {}", expr_node),
        }
    }
}

impl fmt::Display for StatementNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.statement)
    }
}

impl fmt::Display for FunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Function: {} -> {:?}", self.name, self.return_value)?;
        for statement_node in &self.body {
            writeln!(f, "{}", statement_node)?;
        }
        Ok(())
    }
}

impl fmt::Display for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function_node in &self.body {
            writeln!(f, "{}", function_node)?;
        }
        Ok(())
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parser: token_index = {}, token_stream = {:?}",
            self.token_index, self.token_stream
        )
    }
}
