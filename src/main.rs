use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

mod lexer;
mod parser;
mod x86_64_generator;
mod llvm_ir_generator;

#[derive(Debug, PartialEq, Clone, Copy)]
enum CompilerStage {
    Lex,
    Parse,
    LLVMGen,
    CodeGen,
    Compile,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_args(&args) {
        Ok((stage, input_file)) => {
            let input_path = Path::new(&input_file);
            if !input_path.exists() {
                eprintln!("Error: Input file does not exist: {}", input_file);
                process::exit(1);
            }

            match process_file(stage, input_path) {
                Ok(output_path) => {
                    println!("Compilation completed. Output: {}", output_path.display());
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn parse_args(args: &[String]) -> Result<(CompilerStage, String), String> {
    match args.len() {
        2 => Ok((CompilerStage::Compile, args[1].clone())),
        3 => {
            let stage = match args[1].as_str() {
                "--lex" => CompilerStage::Lex,
                "--parse" => CompilerStage::Parse,
                "--llvm" => CompilerStage::LLVMGen,
                "--codegen" => CompilerStage::CodeGen,
                _ => return Err(format!("Unknown option: {}", args[1])),
            };
            Ok((stage, args[2].clone()))
        }
        _ => Err(format!(
            "Usage: {} [--lex|--parse|--llvm|--codegen] <input_file>",
            args[0]
        )),
    }
}

fn process_file(target_stage: CompilerStage, input_path: &Path) -> Result<PathBuf, String> {
    let input = fs::read_to_string(input_path).map_err(|e| format!("Failed to read input file: {}", e))?;

    let tokens = lexer::tokenize(&input).map_err(|e| format!("Lexing failed: {}", e))?;
    if target_stage == CompilerStage::Lex {
        println!("Lexing completed. Tokens:\n{:?}", tokens);
        return Ok(input_path.to_path_buf());
    }

    let c_ast = parser::parse(tokens).map_err(|e| format!("Parsing failed: {}", e))?;
    if target_stage == CompilerStage::Parse {
        println!("Parsing completed. AST:\n{}", c_ast);
        return Ok(input_path.to_path_buf());
    }

    let llvm_ir_ast = llvm_ir_generator::generate(&c_ast).map_err(|e| format!("LLVM IR generation failed: {}", e))?;
    if target_stage == CompilerStage::LLVMGen {
        let llvm_file = input_path.with_extension("ll");
        fs::write(&llvm_file, format!("{}", llvm_ir_ast))
            .map_err(|e| format!("Failed to write LLVM IR file: {}", e))?;
        println!("LLVM IR file generated: {}", llvm_file.display());
        return Ok(llvm_file);
    }

    let x86_64_ast = x86_64_generator::generate(&llvm_ir_ast).map_err(|e| format!("Code generation failed: {}", e))?;
    if target_stage == CompilerStage::CodeGen {
        println!("Code generation completed");
        return Ok(input_path.to_path_buf());
    }

    let assembly = format!("{}", x86_64_ast);
    let assembly_file = input_path.with_extension("s");
    fs::write(&assembly_file, assembly).map_err(|e| format!("Failed to write assembly file: {}", e))?;

    println!("Assembly file generated: {}", assembly_file.display());
    Ok(assembly_file)
}