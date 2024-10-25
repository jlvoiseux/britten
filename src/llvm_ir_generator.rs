use std::fmt;
use crate::parser::{Program, FunctionDefinition, Statement, Expression, UnaryOperator};
use std::sync::atomic::{AtomicUsize, Ordering};

static REGISTER_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_register() -> String {
    format!("%{}", REGISTER_COUNTER.fetch_add(1, Ordering::SeqCst))
}

#[derive(Debug, Clone)]
pub enum LLVMConstruct {
    Module(LLVMFunction),
}

#[derive(Debug, Clone)]
pub enum LLVMFunction {
    Function {
        name: String,
        instructions: Vec<LLVMInstruction>,
    },
}

#[derive(Debug, Clone)]
pub enum LLVMInstruction {
    ReturnValue(LLVMValue),
    Store(LLVMValue, LLVMValue),
    Load(String, LLVMValue),
    Alloca(String, String),
    UnaryOp(String, String, LLVMUnaryOp, LLVMValue),
}

#[derive(Debug, Clone)]
pub enum LLVMUnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum LLVMValue {
    Register(String),
    Immediate(i32),
}

pub fn generate(program: &Program) -> Result<LLVMConstruct, String> {
    match program {
        Program::Program(function) => Ok(LLVMConstruct::Module(generate_function(function)?)),
    }
}

fn generate_function(function: &FunctionDefinition) -> Result<LLVMFunction, String> {
    match function {
        FunctionDefinition::Function { name, body } => {
            let instructions = generate_statement(body)?;
            Ok(LLVMFunction::Function {
                name: name.clone().to_string(),
                instructions,
            })
        }
    }
}

fn generate_statement(statement: &Statement) -> Result<Vec<LLVMInstruction>, String> {
    REGISTER_COUNTER.store(0, Ordering::SeqCst);
    match statement {
        Statement::Return(expr) => {
            let (mut instructions, final_reg) = generate_expression(expr)?;
            instructions.push(LLVMInstruction::ReturnValue(LLVMValue::Register(final_reg)));
            Ok(instructions)
        }
    }
}

fn generate_expression(expr: &Expression) -> Result<(Vec<LLVMInstruction>, String), String> {
    match expr {
        Expression::Constant(value) => {
            let alloca_reg = next_register();
            let load_reg = next_register();
            let instructions = vec![
                LLVMInstruction::Alloca(alloca_reg.clone(), "i32".to_string()),
                LLVMInstruction::Store(
                    LLVMValue::Immediate(*value),
                    LLVMValue::Register(alloca_reg.clone()),
                ),
                LLVMInstruction::Load(
                    load_reg.clone(),
                    LLVMValue::Register(alloca_reg.clone()),
                ),
            ];
            Ok((instructions, load_reg))
        }
        Expression::Unary(op, inner_expr) => {
            let (mut instructions, prev_reg) = generate_expression(inner_expr)?;
            let result_reg = next_register();

            let op_inst = match op {
                UnaryOperator::Complement => LLVMInstruction::UnaryOp(
                    result_reg.clone(),
                    "i32".to_string(),
                    LLVMUnaryOp::Not,
                    LLVMValue::Register(prev_reg),
                ),
                UnaryOperator::Negate => LLVMInstruction::UnaryOp(
                    result_reg.clone(),
                    "i32".to_string(),
                    LLVMUnaryOp::Neg,
                    LLVMValue::Register(prev_reg),
                ),
            };
            instructions.push(op_inst);
            Ok((instructions, result_reg))
        }
    }
}

impl fmt::Display for LLVMConstruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLVMConstruct::Module(func) => {
                writeln!(f, "{}", func)
            }
        }
    }
}

impl fmt::Display for LLVMFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLVMFunction::Function { name, instructions } => {
                writeln!(f, "define i32 @{}() {{", name)?;
                writeln!(f, "entry:")?;
                for instruction in instructions {
                    write!(f, "    {}", instruction)?;
                }
                writeln!(f, "}}")
            }
        }
    }
}

impl fmt::Display for LLVMInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLVMInstruction::ReturnValue(value) => writeln!(f, "ret i32 {}", value),
            LLVMInstruction::Store(src, dst) => writeln!(f, "store i32 {}, i32* {}", src, dst),
            LLVMInstruction::Load(dst, src) => writeln!(f, "{} = load i32, i32* {}", dst, src),
            LLVMInstruction::Alloca(dst, ty) => writeln!(f, "{} = alloca {}", dst, ty),
            LLVMInstruction::UnaryOp(dst, ty, op, value) => {
                match op {
                    LLVMUnaryOp::Not => writeln!(f, "{} = xor {} {}, -1", dst, ty, value),
                    LLVMUnaryOp::Neg => writeln!(f, "{} = sub {} 0, {}", dst, ty, value),
                }
            }
        }
    }
}

impl fmt::Display for LLVMValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLVMValue::Register(reg) => write!(f, "{}", reg),
            LLVMValue::Immediate(value) => write!(f, "{}", value),
        }
    }
}