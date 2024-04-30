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
            Expr::BinaryOp(operator, left_expr, right_expr) => {
                match operator {
                    TokenType::Plus => {
                        let mut addition_asm = "".to_string();

                        // Values will lie in x0
                        let add_left_expr_asm = left_expr.generate_assembly();
                        let add_right_expr_asm = right_expr.generate_assembly();

                        // Store left_expr_asm on stack, will lie in x0,
                        addition_asm += &format!(
                        "{}\n\tstr x0, [sp, #-8]\n\t{}\n\tldr x1, [sp, #-8]\n\tadd x0, x0, x1\n\t",
                        add_left_expr_asm, add_right_expr_asm
                    );

                        addition_asm
                    }
                    TokenType::Minus => {
                        let mut subtraction_asm = "".to_string();

                        let sub_left_expr_asm = left_expr.generate_assembly();
                        let sub_right_expr_asm = right_expr.generate_assembly();

                        subtraction_asm += &format!("{}\n\tstr x0, [sp, #-8]\n\t{}\n\tldr x1, [sp, #-8]\n\tsub x0, x1, x0\n\t", sub_left_expr_asm, sub_right_expr_asm);

                        subtraction_asm
                    }
                    TokenType::Mul => {
                        let mut multiplication_asm = "".to_string();

                        let mul_left_expr_asm = left_expr.generate_assembly();
                        let mul_right_expr_asm = right_expr.generate_assembly();

                        multiplication_asm += &format!(
                            "{}\n\tstr x0, [sp, #-8]\n\t{}\n\tldr x1, [sp, #-8]\n\tmul x0, x1, x0",
                            mul_left_expr_asm, mul_right_expr_asm
                        );

                        multiplication_asm
                    }
                    TokenType::Div => unimplemented!(),
                    _ => format!("Unsupported operator: {}", operator),
                }
            }
        }
    }
}

impl ProgramNode {
    pub fn generate_assembly(&self) -> String {
        let mut program_body_asm = "\t.text\n".to_string(); // Boilerplate to define texe-section of prog

        // Prepending function names
        for function_name in self.function_names.clone() {
            program_body_asm += &format!("\t.global _{}\n", function_name);
        }

        // Generating assembly for instructions
        for function in self.body.clone() {
            program_body_asm += &format!("\n_{}:{}\n", function.name, function.generate_assembly());
        }

        program_body_asm
    }
}

impl FunctionNode {
    pub fn generate_assembly(&self) -> String {
        let mut function_body_asm = "".to_string();
        for statement in &self.body {
            function_body_asm += &format!("\n\t{}", statement.generate_assembly());
        }

        function_body_asm
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
