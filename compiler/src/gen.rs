use crate::{
    parser::{Expr, ExprNode, FunctionNode, ProgramNode, Statement, StatementNode},
    token::TokenType,
};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

// A list, treated as a stack to keep track of labels where they are needed, will only contain the last used number of the label
// Assumes scheme .L1, .L2 etc
// Is used f.ex to short circ && or ||
lazy_static! {
    static ref CURRENT_OCCUPIED_LABEL_NUMBER: Arc<Mutex<Vec<i32>>> = {
        let stack = Mutex::new(vec![1]);
        Arc::new(stack)
    };
}

impl ExprNode {
    pub fn generate_assembly(&self) -> String {
        match &self.expr {
            Expr::Number(num) => format!("\n\tmov w0, #{}", num),

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
                    format!(
                        "{}\n\tcmp x0, #0
                        \n\tmov x0, #0
                        \n\tcset x0, eq",
                        expr_asm
                    )
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
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]{}\n\tldr x1, [sp, 12]\n\tadd x0, x0, x1\n\tadd sp, sp, 16",
                            add_left_expr_asm, add_right_expr_asm
                        );

                        addition_asm
                    }
                    TokenType::Minus => {
                        let mut subtraction_asm = "".to_string();

                        let sub_left_expr_asm = left_expr.generate_assembly();
                        let sub_right_expr_asm = right_expr.generate_assembly();

                        subtraction_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]\n{}\n\tldr x1, [sp, 12]\n\tsub x0, x1, x0\n\tadd sp, sp, 16",
                            sub_left_expr_asm, sub_right_expr_asm
                        );

                        subtraction_asm
                    }
                    TokenType::Mul => {
                        let mut multiplication_asm = "".to_string();

                        let mul_left_expr_asm = left_expr.generate_assembly();
                        let mul_right_expr_asm = right_expr.generate_assembly();

                        multiplication_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tmul x0, x1, x0\n\tadd sp, sp, 16",
                            mul_left_expr_asm, mul_right_expr_asm
                        );

                        multiplication_asm
                    }

                    TokenType::Div => {
                        let mut division_asm = "".to_string();

                        let div_left_expr_asm = left_expr.generate_assembly();
                        let div_right_expr_asm = right_expr.generate_assembly();

                        division_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tsdiv x0, x1, x0\n\tadd sp, sp, 16",
                            div_left_expr_asm, div_right_expr_asm
                        );

                        division_asm
                    }

                    TokenType::Or => {
                        let mut or_asm = "".to_string();

                        let or_left_expr_asm = left_expr.generate_assembly();
                        let or_right_expr_asm = right_expr.generate_assembly();

                        let mut stack = CURRENT_OCCUPIED_LABEL_NUMBER.lock().unwrap();

                        // Bør returnere 1 i første omgang
                        let stack_top = stack.pop();
                        let free_label = stack_top.unwrap() + 1; // 2 for første label

                        or_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tcmp x1, 0\n\tbne .L{}\n\tcmp x0, 0\n\tbeq .L{}\n.L{}:\n\tmov x0, 1\n\tb   .L{}\n.L{}:\n\tmov x0, 0\n.L{}:\n\tadd sp, sp, 16",
                            or_left_expr_asm,
                            or_right_expr_asm,
                            free_label,
                            free_label + 1,
                            free_label,
                            free_label + 2,
                            free_label + 1,
                            free_label + 2
                        );

                        stack.push(free_label + 2); // Vil være sist brukte label
                        or_asm
                    }

                    TokenType::And => {
                        let mut and_asm = "".to_string();

                        let and_left_expr_asm = left_expr.generate_assembly();
                        let and_right_expr_asm = right_expr.generate_assembly();

                        let mut stack = CURRENT_OCCUPIED_LABEL_NUMBER.lock().unwrap();

                        let stack_top = stack.pop();
                        let free_label = stack_top.unwrap() + 1; // 5 i første omgang, kanskje

                        and_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tcmp x1, 0\n\tbeq .L{}\n\tcmp x0, 0\n\tbeq .L{}\n\tmov x0, 1\n\tb   .L{}\n.L{}:\n\tmov w0, 0\n.L{}:\n\tadd sp, sp, 16",
                            and_left_expr_asm,
                            and_right_expr_asm,
                            free_label,
                            free_label,
                            free_label + 1,
                            free_label,
                            free_label + 1
                        );

                        stack.push(free_label + 1);
                        and_asm
                    }
                    TokenType::Eq => {
                        let mut equal_asm = "".to_string();

                        let equal_left_expr_asm = left_expr.generate_assembly();
                        let equal_right_expr_asm = right_expr.generate_assembly();

                        equal_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tcmp x1, x0\n\tcset x0, eq\n\tadd sp, sp, 16",
                            equal_left_expr_asm, equal_right_expr_asm
                        );

                        equal_asm
                    }
                    TokenType::Neq => {
                        let mut not_equal_asm = "".to_string();

                        let not_equal_left_expr_asm = left_expr.generate_assembly();
                        let not_equal_right_expr_asm = right_expr.generate_assembly();

                        not_equal_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tcmp x0, x1\n\tcset x0, ne\n\tadd sp, sp, 16",
                            not_equal_left_expr_asm, not_equal_right_expr_asm
                        );

                        not_equal_asm
                    }
                    TokenType::Lt => {
                        let mut less_than_asm = "".to_string();

                        let less_than_left_expr_asm = left_expr.generate_assembly();
                        let less_than_right_expr_asm = right_expr.generate_assembly();

                        let mut stack = CURRENT_OCCUPIED_LABEL_NUMBER.lock().unwrap();

                        let stack_top = stack.pop();
                        let free_label = stack_top.unwrap() + 1;

                        less_than_asm += &format!(
                            "{}\n\tsub sp, sp, 16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tcmp x1, x0\n\tblt .L{}\n\tmov w0, 0\n\tb .L{}\n.L{}:\n\tmov w0, 1\n.L{}:\n\tadd sp, sp, 16\n\t",
                            less_than_left_expr_asm,
                            less_than_right_expr_asm,
                            free_label,
                            free_label + 1,
                            free_label,
                            free_label + 1
                        );

                        stack.push(free_label + 1);
                        less_than_asm
                    }
                    TokenType::Gt => {
                        let mut greater_than_asm = "".to_string();

                        let greater_than_left_expr_asm = left_expr.generate_assembly();
                        let greater_than_right_expr_asm = right_expr.generate_assembly();
                        let mut stack = CURRENT_OCCUPIED_LABEL_NUMBER.lock().unwrap();

                        let stack_top = stack.pop();
                        let free_label = stack_top.unwrap() + 1;

                        greater_than_asm += &format!(
                            "{}\n\tsub sp, sp, 16\n\tstr x0, [sp, 12]\n\t{}\n\tldr x1, [sp, 12]\n\tcmp x1, x0\n\tbgt .L{}\n\tmov w0, 0\n\t b .L{}\n.L{}:\n\tmov w0, 1\n.L{}:\n\tadd sp, sp, 16",
                            greater_than_left_expr_asm,
                            greater_than_right_expr_asm,
                            free_label,
                            free_label + 1,
                            free_label,
                            free_label + 1
                        );

                        stack.push(free_label + 1);
                        greater_than_asm
                    }
                    TokenType::Le => {
                        let mut less_eq_than_asm = "".to_string();

                        let less_eq_than_left_expr_asm = left_expr.generate_assembly();
                        let less_eq_than_right_expr_asm = right_expr.generate_assembly();
                        let mut stack = CURRENT_OCCUPIED_LABEL_NUMBER.lock().unwrap();

                        let stack_top = stack.pop();
                        let free_label = stack_top.unwrap() + 1;

                        less_eq_than_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 16]\n\t{}\n\tldr x1, [sp, 16]\n\tcmp x1, x0\n\tble .L{}\n\tmov x0, 0\n\tb   .L{}\n.L{}:\n\tmov x0, 1\n.L{}:\n\tadd sp, sp, 16",
                            less_eq_than_left_expr_asm,
                            less_eq_than_right_expr_asm,
                            free_label,
                            free_label + 1,
                            free_label,
                            free_label + 1
                        );

                        stack.push(free_label + 1);
                        less_eq_than_asm
                    }
                    TokenType::Ge => {
                        let mut greater_eq_than_asm = "".to_string();

                        let greater_eq_than_left_expr_asm = left_expr.generate_assembly();
                        let greater_eq_than_right_expr_asm = right_expr.generate_assembly();
                        let mut stack = CURRENT_OCCUPIED_LABEL_NUMBER.lock().unwrap();

                        let stack_top = stack.pop();
                        let free_label = stack_top.unwrap() + 1;

                        greater_eq_than_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr x0, [sp, 16]\n\t{}\n\tldr x1, [sp, 16]\n\tcmp x1, x0\n\tbge .L{}\n\tmov x0, 0\n\tb   .L{}\n.L{}:\n\tmov x0, 1\n.L{}:\n\tadd sp, sp, 16",
                            greater_eq_than_left_expr_asm,
                            greater_eq_than_right_expr_asm,
                            free_label,
                            free_label + 1,
                            free_label,
                            free_label + 1
                        );

                        stack.push(free_label + 1);
                        greater_eq_than_asm
                    }
                    _ => format!("Unsupported operator: {}", operator),
                }
            }
            _ => format!("Not yet implemented!"),
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

            Statement::Assignment(
                TokenType::IntKeyword,
                TokenType::Identifier,
                TokenType::Assign,
                expr_node,
            ) => {
                let expr_asm = expr_node.as_ref().unwrap().generate_assembly();
                format!(
                    "\n\tsub sp, sp, #16
                    \n\t{}
                    \n\t
                    str x0, [sp,12]
                    \n\tmov x0, 0
                    \n\tadd sp, sp, 16\n\t",
                    expr_asm
                )
            }
            Statement::Assignment(
                TokenType::CharKeyword,
                TokenType::Identifier,
                TokenType::Assign,
                expr_node,
            ) => {
                let expr_asm = expr_node.as_ref().unwrap().generate_assembly();
                format!(
                    "\n\tsub sp, sp, #16
                    \n\t{}
                    \n\t
                    str x0, [sp,12]
                    \n\tmov x0, 0
                    \n\tadd sp, sp, 16\n\t",
                    expr_asm
                )
            }
            _ => "Unsupported statement".to_string(),
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
        assembly += &self.root.generate_assembly();

        assembly
    }
}
