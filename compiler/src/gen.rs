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

impl FunctionNode {
    pub fn generate_assembly(&self) -> String {
        let mut function_text_global_name =
            format!("\t.text\n\t.global _{}\n_{}:", self.name, self.name);
        for statement in &self.body {
            function_text_global_name = format!(
                "{}\n\t{}",
                function_text_global_name,
                statement.generate_assembly()
            );
        }

        function_text_global_name
    }
}

impl StatementNode {
    pub fn generate_assembly(&self) -> String {
        match &self.statement {
            Statement::Return(expr_node) => {
                let expr_asm = expr_node.generate_assembly();
                format!("{}\n\tret", expr_asm)
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

    pub fn walk_da_tree(&self) -> String {
        let mut assembly = "".to_string();
        for function in &self.root.body {
            assembly += &function.generate_assembly();
        }

        assembly
    }
}
