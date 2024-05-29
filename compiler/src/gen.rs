use crate::{
    parser::{Expr, ExprNode, FunctionNode, ProgramNode, Statement, StatementNode}, token::TokenType
};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::symbol_table::SymbolTable;

// A list, treated as a stack to keep track of labels where they are needed, will only contain the last used number of the label
// Assumes scheme .L1, .L2 etc
// Is used f.ex to short circ && or ||
lazy_static! {
    static ref CURRENT_OCCUPIED_LABEL_NUMBER: Arc<Mutex<Vec<i32>>> = {
        let stack = Mutex::new(vec![1]);
        Arc::new(stack)
    };
}

//Shared internal symbol_table reference for generator
lazy_static! {
    static ref SYMBOL_TABLE_GENERATOR: Arc<Mutex<SymbolTable>> = {
        let symbol_table = Mutex::new(SymbolTable::new());
        Arc::new(symbol_table)
    };
}


impl ExprNode {
    pub fn generate_assembly(&self) -> String {
        match &self.expr {
            Expr::Number(num) => format!("\n\tmov w0, #{}", num),

            Expr::UnaryOp(operator, expr) => match operator {
                TokenType::Minus => {
                    let expr_asm = expr.as_ref().unwrap().generate_assembly();
                    format!("{}\n\tneg w0, w0", expr_asm)
                }
                TokenType::BitComplement => {
                    let expr_asm = expr.as_ref().unwrap().generate_assembly();
                    format!("{}\n\tmvn w0, w0", expr_asm)
                }
                TokenType::Not => {
                    let expr_asm = expr.as_ref().unwrap().generate_assembly();
                    format!("{}\n\tcmp w0, #0\n\tmov w0, #0\n\tcset w0, eq", expr_asm)
                }

                _ => "Unsupported operator".to_string(),
            },
            Expr::BinaryOp(operator, left_expr, right_expr) => {



                match operator {
                    TokenType::Plus => {
                        let mut addition_asm = "".to_string();

                        // Values will lie in x0
                        let plus_right_expr_asm = right_expr.generate_assembly();
                        let plus_left_expr_asm = left_expr.generate_assembly();

                        // Store left_expr_asm on stack, will lie in x0,
                        addition_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]{}\n\tldr w1, [sp, 12]\n\tadd w0, w0, w1\n\tadd sp, sp, 16",
                            plus_left_expr_asm, plus_right_expr_asm
                        );

                        addition_asm
                    }
                    TokenType::Minus => {
                        let mut subtraction_asm = "".to_string();

                        let sub_left_expr_asm = left_expr.generate_assembly();
                        let sub_right_expr_asm = right_expr.generate_assembly();

                        subtraction_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]\n{}\n\tldr w1, [sp, 12]\n\tsub w0, w1, w0\n\tadd sp, sp, 16",
                            sub_left_expr_asm, sub_right_expr_asm
                        );

                        subtraction_asm
                    }
                    TokenType::Mul => {
                        let mut multiplication_asm = "".to_string();

                        let mul_left_expr_asm = left_expr.generate_assembly();
                        let mul_right_expr_asm = right_expr.generate_assembly();

                        multiplication_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tmul w0, w1, w0\n\tadd sp, sp, 16",
                            mul_left_expr_asm, mul_right_expr_asm
                        );

                        multiplication_asm
                    }

                    TokenType::Div => {
                        let mut division_asm = "".to_string();

                        let div_left_expr_asm = left_expr.generate_assembly();
                        let div_right_expr_asm = right_expr.generate_assembly();

                        division_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tsdiv w0, w1, w0\n\tadd sp, sp, 16",
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
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tcmp w1, 0\n\tbne .L{}\n\tcmp w0, 0\n\tbeq .L{}\n.L{}:\n\tmov w0, 1\n\tb .L{}\n.L{}:\n\tmov w0, 0\n.L{}:\n\tadd sp, sp, 16",
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
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tcmp w1, 0\n\tbeq .L{}\n\tcmp w0, 0\n\tbeq .L{}\n\tmov w0, 1\n\tb .L{}\n.L{}:\n\tmov w0, 0\n.L{}:\n\tadd sp, sp, 16",
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
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tcmp w1, w0\n\tcset w0, eq\n\tadd sp, sp, 16",
                            equal_left_expr_asm, equal_right_expr_asm
                        );

                        equal_asm
                    }
                    TokenType::Neq => {
                        let mut not_equal_asm = "".to_string();

                        let not_equal_left_expr_asm = left_expr.generate_assembly();
                        let not_equal_right_expr_asm = right_expr.generate_assembly();

                        not_equal_asm += &format!(
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tcmp w0, w1\n\tcset w0, ne\n\tadd sp, sp, 16",
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
                            "{}\n\tsub sp, sp, 16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tcmp w1, w0\n\tblt .L{}\n\tmov w0, 0\n\tb .L{}\n.L{}:\n\tmov w0, 1\n.L{}:\n\tadd sp, sp, 16\n\t",
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
                            "{}\n\tsub sp, sp, 16\n\tstr w0, [sp, 12]\n\t{}\n\tldr w1, [sp, 12]\n\tcmp w1, w0\n\tbgt .L{}\n\tmov w0, 0\n\t b .L{}\n.L{}:\n\tmov w0, 1\n.L{}:\n\tadd sp, sp, 16",
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
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 16]\n\t{}\n\tldr w1, [sp, 16]\n\tcmp w1, w0\n\tble .L{}\n\tmov w0, 0\n\tb .L{}\n.L{}:\n\tmov w0, 1\n.L{}:\n\tadd sp, sp, 16",
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
                            "{}\n\tsub sp, sp, #16\n\tstr w0, [sp, 16]\n\t{}\n\tldr w1, [sp, 16]\n\tcmp w1, w0\n\tbge .L{}\n\tmov w0, 0\n\tb .L{}\n.L{}:\n\tmov w0, 1\n.L{}:\n\tadd sp, sp, 16",
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
            // Identifier is an enum variant, token_assign = '=' and expr is an expr
            /*
               Need to check for None values to see what needs to be generated.
               if Identifier = None, then only generate arm64 for the expression
               else generate variable data, store the evaluated expression on the stack
               and create an entry in the symbol table either here or in the parser, unsure
            */
            Expr::DeclAssign(identifier, _token_assign, expr) => {
                let mut decl_assign_asm = "".to_string();

                if let Some(expr_node) = expr {
                    let expr_asm = expr_node.generate_assembly();
                    decl_assign_asm.push_str(&expr_asm);
                }

                if let Some(identifier) = identifier {
                    if let Expr::Identifier(ident_str) = &identifier.expr {
                        let symbol_table = SYMBOL_TABLE_GENERATOR.lock().unwrap();
                        match symbol_table.lookup_entry(ident_str) {
                            Some(entry) => {
                                  decl_assign_asm.push_str(&format!("\n\tstr w0, [sp, #{}]", entry.bytes_allocated - entry.stack_offset));
                            },
                            None => println!("No entry found for '{}'", identifier),
                        }
                    }
                }

                decl_assign_asm
            }

            Expr::Identifier(ident) => {
                let mut ident_asm = "".to_string();

                let symbol_table = SYMBOL_TABLE_GENERATOR.lock().unwrap();
                match symbol_table.lookup_entry(ident) {
                    Some(entry) => {
                        if entry.is_initialized {
                            let variable_location =  symbol_table.current_bytes_allocated - entry.stack_offset;
                            ident_asm.push_str(&format!("\n\tldr w0, [sp, {}]", variable_location));
                        }
                    }
                    None => println!("No entry found for '{}'", ident),
                }
                ident_asm
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
                let mut return_asm = "".to_string();
                let stack_offset: u32;
                let bytes_allocated: u32;

                // Scope to force symbol_table out of scope
                {
                    let symbol_table = SYMBOL_TABLE_GENERATOR.lock().unwrap();
                    stack_offset = symbol_table.current_stack_offset;
                    bytes_allocated = symbol_table.current_bytes_allocated;
                }

                let expr_asm = expr_node.generate_assembly();

                return_asm.push_str(&expr_asm);

                if stack_offset > 0 {

                       return_asm.push_str(&format!("\n\tadd sp, sp, #{}", bytes_allocated));
                }

                return_asm.push_str("\n\tret");

                return_asm
            }
            Statement::Assignment(
                TokenType::IntKeyword,
                _token,
                Some(TokenType::Assign),
                expr_node,
            ) => {
                let mut assign_asm = "".to_string();
                let mut symbol_table = SYMBOL_TABLE_GENERATOR.lock().unwrap();

                let stack_offset = symbol_table.current_stack_offset;
                let mut total_bytes_allocated = symbol_table.current_bytes_allocated;

                if(total_bytes_allocated - stack_offset) == 0 || stack_offset == 0 {
                    println!("Bytes allocated on the stack: {}", symbol_table.current_bytes_allocated);
                    assign_asm.push_str(&format!("\n\tsub sp, sp, #16"));
                    symbol_table.allocate_space();
                    println!("Bytes allocated on the stack: {}", symbol_table.current_bytes_allocated);
                }

                let expr_asm = expr_node.as_ref().unwrap().generate_assembly();
                total_bytes_allocated = symbol_table.current_bytes_allocated; // Need to update the variable
                assign_asm.push_str(&expr_asm);
                if _token.token_type == TokenType::Identifier {
                    let token_value = _token.value.clone();
                    if let Some(token_value) = token_value {

                        symbol_table.add_entry(token_value, true);
                        let stack_offset = symbol_table.current_stack_offset;
                        let stack_offset_to_store_variable = total_bytes_allocated - stack_offset;
                        //16 - stack_offset
                        assign_asm.push_str(&format!("\n\tstr w0, [sp, #{}]", stack_offset_to_store_variable));
                    }
                }

                assign_asm
            }
            Statement::Assignment(TokenType::IntKeyword, _token, _opt_assign, _opt_expr_node) => {
                let mut assign_asm = "".to_string();
                let mut symbol_table = SYMBOL_TABLE_GENERATOR.lock().unwrap();

                let stack_offset = symbol_table.current_stack_offset;
                let total_bytes_allocated = symbol_table.current_bytes_allocated;
                if(total_bytes_allocated - stack_offset) == 0 || stack_offset == 0 {
                    assign_asm.push_str(&format!("\n\tsub sp, sp, #16"));
                    symbol_table.allocate_space();
                }

                if _token.token_type == TokenType::Identifier {
                    let token_value = _token.value.clone();
                    if let Some(token_value) = token_value {
                        symbol_table.add_entry(token_value, false);
                    }
                }

                assign_asm
            }
            Statement::Assignment(
                TokenType::CharKeyword,
                _token, // Not needed
                Some(TokenType::Assign),
                expr_node,
            ) => {
                let expr_asm = expr_node.as_ref().unwrap().generate_assembly();
                format!(
                    "\n\tsub sp, sp, #16\n\t{}\n\tstr w0, [sp,12]\n\tmov w0, 0\n\tadd sp, sp, 16\n\t",
                    expr_asm
                )
            }
            Statement::DeclAssignForStmnt(expr) => {
                let expr_asm = expr.as_ref().unwrap().generate_assembly();
                expr_asm
            }
            _ => "Unsupported statement".to_string(),
        }
    }
}


pub struct Generator {
    pub root: ProgramNode,
    pub symbol_table: Arc<Mutex<SymbolTable>>,
}

impl Generator {
    pub fn new(root_node: ProgramNode, symbol_table_thread_safe: Arc<Mutex<SymbolTable>>) -> Self {
        Generator { 
            root: root_node,
            symbol_table: symbol_table_thread_safe,
        }
    }

    pub fn walk_da_tree(&self) -> String {
        let mut assembly = "".to_string();
        assembly += &self.root.generate_assembly();

        assembly
    }
}
