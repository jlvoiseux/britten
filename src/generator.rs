use std::fmt;
use crate::parser::{Program, FunctionDefinition, Statement, Expression};

pub enum AssemblyConstruct {
    Program(AssemblyFunction),
}

pub enum AssemblyFunction {
    Function {
        name: String,
        instructions: Vec<AssemblyInstruction>,
    },
}

pub enum AssemblyInstruction {
    Mov(AssemblyOperand, AssemblyOperand),
    Ret,
}

pub enum AssemblyOperand {
    Register(String),
    Immediate(i32),
}

pub fn generate(program: &Program) -> Result<AssemblyConstruct, String> {
    match program {
        Program::Program(function) => Ok(AssemblyConstruct::Program(generate_function(function)?)),
    }
}

fn generate_function(function: &FunctionDefinition) -> Result<AssemblyFunction, String> {
    match function {
        FunctionDefinition::Function { name, body } => {
            let instructions = generate_statement(body)?;
            Ok(AssemblyFunction::Function {
                name: name.clone().to_string(),
                instructions,
            })
        }
    }
}

fn generate_statement(statement: &Statement) -> Result<Vec<AssemblyInstruction>, String> {
    match statement {
        Statement::Return(expr) => {
            let mut instructions = generate_expression(expr)?;
            instructions.push(AssemblyInstruction::Ret);
            Ok(instructions)
        }
    }
}

fn generate_expression(expr: &Expression) -> Result<Vec<AssemblyInstruction>, String> {
    match expr {
        Expression::Constant(value) => {
            Ok(vec![AssemblyInstruction::Mov(
                AssemblyOperand::Immediate(*value),
                AssemblyOperand::Register("eax".to_string()),
            )])
        }
    }
}

pub trait PrettyPrint {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result;
}

impl fmt::Display for AssemblyConstruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl PrettyPrint for AssemblyConstruct {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            AssemblyConstruct::Program(func) => {
                func.pretty_print(f, level)?;
                writeln!(f, ".section .note.GNU-stack,\"\",@progbits")
            }
        }
    }
}

impl PrettyPrint for AssemblyFunction {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            AssemblyFunction::Function { name, instructions } => {
                writeln!(f, ".globl {}", name)?;
                writeln!(f, "{}:", name)?;
                for instruction in instructions {
                    instruction.pretty_print(f, level + 1)?;
                }
                Ok(())
            }
        }
    }
}

impl PrettyPrint for AssemblyInstruction {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            AssemblyInstruction::Mov(src, dst) => {
                writeln!(f, "{}movl {}, {}", indent(level), src, dst)
            }
            AssemblyInstruction::Ret => writeln!(f, "{}ret", indent(level)),
        }
    }
}

impl fmt::Display for AssemblyOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyOperand::Register(reg) => write!(f, "%{}", reg),
            AssemblyOperand::Immediate(value) => write!(f, "${}", value),
        }
    }
}

fn indent(level: usize) -> String {
    "    ".repeat(level)
}