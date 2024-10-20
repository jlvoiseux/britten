use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

mod lexer;
mod parser;
mod generator;

use lexer::tokenize;
use parser::parse;
use generator::generate;

#[derive(Debug, PartialEq, Clone, Copy)]
enum CompilerStage {
    Lex,
    Parse,
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
                "--codegen" => CompilerStage::CodeGen,
                _ => return Err(format!("Unknown option: {}", args[1])),
            };
            Ok((stage, args[2].clone()))
        }
        _ => Err(format!(
            "Usage: {} [--lex|--parse|--codegen] <input_file>",
            args[0]
        )),
    }
}

fn process_file(target_stage: CompilerStage, input_path: &Path) -> Result<PathBuf, String> {
    let input = fs::read_to_string(input_path).map_err(|e| format!("Failed to read input file: {}", e))?;

    let tokens = tokenize(&input).map_err(|e| format!("Lexing failed: {}", e))?;
    if target_stage == CompilerStage::Lex {
        println!("Lexing completed. Tokens:\n{:?}", tokens);
        return Ok(input_path.to_path_buf());
    }

    let ast = parse(tokens).map_err(|e| format!("Parsing failed: {}", e))?;
    if target_stage == CompilerStage::Parse {
        println!("Parsing completed. AST:\n{}", ast);
        return Ok(input_path.to_path_buf());
    }

    let assembly_construct = generate(&ast).map_err(|e| format!("Code generation failed: {}", e))?;
    if target_stage == CompilerStage::CodeGen {
        println!("Code generation completed");
        return Ok(input_path.to_path_buf());
    }

    let assembly = format!("{}", assembly_construct);
    let assembly_file = input_path.with_extension("s");
    fs::write(&assembly_file, assembly).map_err(|e| format!("Failed to write assembly file: {}", e))?;

    println!("Assembly file generated: {}", assembly_file.display());
    Ok(assembly_file)
}