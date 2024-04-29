use crate::{
    parser::{Expr, ExprNode, FunctionNode, ProgramNode, Statement, StatementNode},
    token::TokenType,
};

impl ExprNode {
    pub fn generate_assembly(&self) -> String {
        match &self.expr {
            Expr::Number(num) => format!("mov w0, #{}", num),
            Expr::UnaryOp(operator, expr) => match operator {
                TokenType::Minus => {
                    let expr_asm = expr.as_ref().unwrap().generate_assembly();
                    format!("{}\n\tneg x0, x0", expr_asm)
                }
                TokenType::BitComplement => {
                    let expr_asm = expr.as_ref().unwrap().generate_assembly();
                    format!("{}\n\tmvn x0, x0", expr_asm)
                }
                TokenType::Not => {
                    let expr_asm = expr.as_ref().unwrap().generate_assembly();
                    format!("{}\n\tcmp x0, #0\n\tmov x0, #0\n\tcset x0, eq", expr_asm)
                }

                _ => "Unsupported operator".to_string(),
            },
            Expr::BinaryOp(operator, left_expr, right_expr) => match operator {
                TokenType::Plus => unimplemented!(),
                TokenType::Minus => unimplemented!(),
                TokenType::Mul => unimplemented!(),
                TokenType::Div => unimplemented!(),
                _ => format!("Unsupported operator: {}", operator),
            },
        }
    }
}

impl ProgramNode {
    pub fn generate_assembly(&self) -> String {
        let mut program_text_start = "\t.text\n".to_string();

        // Appending function names
        for function_name in self.function_names.clone() {
            program_text_start += &format!("\t.global _{}\n", function_name);
        }

        // Generating assembly for instructions
        for function in self.body.clone() {
            program_text_start +=
                &format!("\n_{}:{}\n", function.name, function.generate_assembly());
        }

        program_text_start
    }
}

impl FunctionNode {
    pub fn generate_assembly(&self) -> String {
        let mut function_text_global_name = "".to_string();
        // format!("\t.text\n\t.global _{}\n_{}:", self.name, self.name);
        for statement in &self.body {
            function_text_global_name = format!(
                "\n\t{}",
                //function_text_global_name,
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
        //for function in &self.root.body {
        //  assembly += &function.generate_assembly();
        //}

        assembly += &self.root.generate_assembly();

        assembly
    }
}
