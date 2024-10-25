use std::fmt;
use crate::llvm_ir_generator::{LLVMConstruct, LLVMFunction, LLVMInstruction, LLVMValue, LLVMUnaryOp};

#[derive(Debug, Clone)]
pub enum AssemblyConstruct {
    Program(AssemblyFunction),
}

#[derive(Debug, Clone)]
pub enum AssemblyFunction {
    Function {
        name: String,
        instructions: Vec<AssemblyInstruction>,
    },
}

#[derive(Debug, Clone)]
pub enum AssemblyInstruction {
    Mov(AssemblyOperand, AssemblyOperand),
    Unary(AssemblyUnaryOperator, AssemblyOperand),
    AllocateStack(i32),
    Ret,
}

#[derive(Debug, Clone)]
pub enum AssemblyUnaryOperator {
    Neg,
    Not
}

#[derive(Debug, Clone)]
pub enum AssemblyOperand {
    Register(AssemblyRegister),
    Immediate(i32),
    PseudoRegister(String),
    StackPointer(i32),
}

#[derive(Debug, Clone)]
pub enum AssemblyRegister {
    AX,
    R10
}

pub fn generate(llvm_ir: &LLVMConstruct) -> Result<AssemblyConstruct, String> {
    match llvm_ir {
        LLVMConstruct::Module(function) => {
            let initial_asm = generate_function(function)?;
            let (stack_allocated_asm, offset) = compute_stack_allocation(&initial_asm)?;
            let fixed_asm = fix_memory_constraints(AssemblyConstruct::Program(stack_allocated_asm), offset.abs())?;
            Ok(fixed_asm)
        }
    }
}


fn generate_function(function: &LLVMFunction) -> Result<AssemblyFunction, String> {
    match function {
        LLVMFunction::Function { name, instructions } => {
            let asm_instructions = instructions.iter()
                .filter_map(|inst| generate_instruction(inst).transpose())
                .collect::<Result<Vec<_>, String>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            Ok(AssemblyFunction::Function {
                name: name.clone(),
                instructions: [asm_instructions, vec![AssemblyInstruction::Ret]].concat(),
            })
        }
    }
}

fn generate_instruction(instruction: &LLVMInstruction) -> Result<Option<Vec<AssemblyInstruction>>, String> {
    match instruction {
        LLVMInstruction::ReturnValue(value) => {
            Ok(Some(vec![AssemblyInstruction::Mov(
                generate_value(value),
                AssemblyOperand::Register(AssemblyRegister::AX)
            )]))
        },
        LLVMInstruction::UnaryOp(dst, _ty, op, value) => {
            let asm_op = match op {
                LLVMUnaryOp::Not => AssemblyUnaryOperator::Not,
                LLVMUnaryOp::Neg => AssemblyUnaryOperator::Neg,
            };

            Ok(Some(vec![
                AssemblyInstruction::Mov(
                    generate_value(value),
                    AssemblyOperand::PseudoRegister(dst.clone())
                ),
                AssemblyInstruction::Unary(
                    asm_op,
                    AssemblyOperand::PseudoRegister(dst.clone())
                )
            ]))
        },
        LLVMInstruction::Store(src, dst) => {
            Ok(Some(vec![AssemblyInstruction::Mov(
                generate_value(src),
                AssemblyOperand::PseudoRegister(dst.to_string())
            )]))
        },
        LLVMInstruction::Load(dst, src) => {
            Ok(Some(vec![AssemblyInstruction::Mov(
                AssemblyOperand::PseudoRegister(src.to_string()),
                AssemblyOperand::PseudoRegister(dst.clone())
            )]))
        },
        LLVMInstruction::Alloca(_dst, _ty) => Ok(None),
    }
}

fn generate_value(value: &LLVMValue) -> AssemblyOperand {
    match value {
        LLVMValue::Register(reg) => AssemblyOperand::PseudoRegister(reg.clone()),
        LLVMValue::Immediate(val) => AssemblyOperand::Immediate(*val),
    }
}

fn replace_pseudo(operand: &AssemblyOperand) -> AssemblyOperand {
    match operand {
        AssemblyOperand::PseudoRegister(reg) => {
            let num = reg.trim_start_matches('%').parse::<i32>().unwrap();
            AssemblyOperand::StackPointer(-4 * (num + 1))
        },
        other => other.clone(),
    }
}

fn compute_stack_allocation(func: &AssemblyFunction) -> Result<(AssemblyFunction, i32), String> {
    match func {
        AssemblyFunction::Function { name, instructions } => {
            let new_instructions = instructions.iter().map(|inst| match inst {
                AssemblyInstruction::Mov(src, dst) => AssemblyInstruction::Mov(
                    replace_pseudo(src),
                    replace_pseudo(dst)
                ),
                AssemblyInstruction::Unary(op, operand) => AssemblyInstruction::Unary(
                    op.clone(),
                    replace_pseudo(operand)
                ),
                other => other.clone(),
            }).collect();

            Ok((AssemblyFunction::Function { name: name.clone(), instructions: new_instructions }, -16))
        }
    }
}

fn fix_memory_constraints(program: AssemblyConstruct, stack_size: i32) -> Result<AssemblyConstruct, String> {
    match program {
        AssemblyConstruct::Program(function) => {
            Ok(AssemblyConstruct::Program(fix_function_memory(function, stack_size)?))
        }
    }
}

fn fix_function_memory(function: AssemblyFunction, stack_size: i32) -> Result<AssemblyFunction, String> {
    match function {
        AssemblyFunction::Function { name, instructions } => {
            let mut new_instructions = Vec::new();
            new_instructions.push(AssemblyInstruction::AllocateStack(stack_size));

            for inst in instructions {
                match inst {
                    AssemblyInstruction::Mov(src, dst) => {
                        if let (AssemblyOperand::StackPointer(_), AssemblyOperand::StackPointer(_)) = (&src, &dst) {
                            new_instructions.push(AssemblyInstruction::Mov(
                                src,
                                AssemblyOperand::Register(AssemblyRegister::R10)
                            ));
                            new_instructions.push(AssemblyInstruction::Mov(
                                AssemblyOperand::Register(AssemblyRegister::R10),
                                dst
                            ));
                        } else {
                            new_instructions.push(AssemblyInstruction::Mov(src, dst));
                        }
                    },
                    other => new_instructions.push(other),
                }
            }

            Ok(AssemblyFunction::Function { name, instructions: new_instructions })
        }
    }
}

impl fmt::Display for AssemblyConstruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyConstruct::Program(func) => {
                write!(f, "{}\n.section .note.GNU-stack,\"\",@progbits", func)
            }
        }
    }
}

impl fmt::Display for AssemblyFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyFunction::Function { name, instructions } => {
                writeln!(f, ".globl {}", name)?;
                writeln!(f, "{}:", name)?;
                writeln!(f, "    pushq %rbp")?;
                writeln!(f, "    movq %rsp, %rbp")?;
                for instruction in instructions {
                    write!(f, "    {}", instruction)?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for AssemblyInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyInstruction::Mov(src, dst) => writeln!(f, "movl {}, {}", src, dst),
            AssemblyInstruction::Unary(op, operand) => writeln!(f, "{} {}", op, operand),
            AssemblyInstruction::AllocateStack(size) => writeln!(f, "subq ${}, %rsp", size),
            AssemblyInstruction::Ret => {
                writeln!(f, "movq %rbp, %rsp")?;
                writeln!(f, "    popq %rbp")?;
                writeln!(f, "    ret")
            }
        }
    }
}

impl fmt::Display for AssemblyUnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyUnaryOperator::Neg => write!(f, "negl"),
            AssemblyUnaryOperator::Not => write!(f, "notl"),
        }
    }
}

impl fmt::Display for AssemblyOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyOperand::Register(reg) => write!(f, "{}", reg),
            AssemblyOperand::Immediate(value) => write!(f, "${}", value),
            AssemblyOperand::PseudoRegister(id) => write!(f, "pseudo({})", id),
            AssemblyOperand::StackPointer(offset) => write!(f, "{}(%rbp)", offset),
        }
    }
}

impl fmt::Display for AssemblyRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyRegister::AX => write!(f, "%eax"),
            AssemblyRegister::R10 => write!(f, "%r10d"),
        }
    }
}