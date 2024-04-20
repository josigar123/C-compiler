use crate::parser::{self, Expr, ExprNode, FunctionNode, ProgramNode, Statement, StatementNode};

impl ExprNode {
    // Assembly for int lit, eller immediate verdi, for mer kompliserte uttrykk
    // kan de matches for andre enum varianter og printe tilsvarende assembly
    pub fn generate_assembly(&self) -> String {
        match &self.expr {
            Expr::Number(num) => format!("mov x0, #{}", num),
        }
    }
}

/*
impl FunctionNode {
    pub fn generate_assembly(&self) -> String {}
}
*/
impl StatementNode {
    pub fn generate_assembly(&self) -> String {
        match &self.statement {
            Statement::Return(expr_node) => {
                let expr_asm = expr_node.generate_assembly();
                format!("{}\nret", expr_asm)
            }
        }
    }
}

pub struct Generator {
    root: ProgramNode,
}

impl Generator {
    pub fn new(root_node: ProgramNode) -> Self {
        Generator { root: root_node }
    }
}
