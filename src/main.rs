use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;

#[derive(Debug)]
enum CompilerOption {
    Lex,
    Parse,
    CodeGen,
    AssemblyOnly,
    FullCompile,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_args(&args) {
        Ok((option, input_file)) => {
            let input_path = Path::new(&input_file);
            if !input_path.exists() {
                eprintln!("Error: Input file does not exist: {}", input_file);
                process::exit(1);
            }
            process_file(option, input_path);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn parse_args(args: &[String]) -> Result<(CompilerOption, String), String> {
    match args.len() {
        2 => Ok((CompilerOption::FullCompile, args[1].clone())),
        3 => {
            let option = match args[1].as_str() {
                "--lex" => CompilerOption::Lex,
                "--parse" => CompilerOption::Parse,
                "--codegen" => CompilerOption::CodeGen,
                "-S" => CompilerOption::AssemblyOnly,
                _ => return Err(format!("Unknown option: {}", args[1])),
            };
            Ok((option, args[2].clone()))
        }
        _ => Err(format!("Usage: {} [--lex|--parse|--codegen|-S] <input_file>", args[0])),
    }
}

fn process_file(option: CompilerOption, input_path: &Path) {
    match option {
        CompilerOption::Lex => lex_file(input_path),
        CompilerOption::Parse => parse_file(input_path),
        CompilerOption::CodeGen => println!("Code generation not yet implemented"),
        CompilerOption::AssemblyOnly => {
            let assembly_file = generate_assembly_file(input_path);
            println!("Assembly file generated: {}", assembly_file.display());
        }
        CompilerOption::FullCompile => {
            let executable = compile(input_path);
            println!("Executable generated: {}", executable.display());
        }
    }
}

fn lex_file(input_path: &Path) {
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    match tokenize(&input) {
        Ok(tokens) => {
            println!("Lexing completed. Tokens:");
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("Lexing error: {}", e);
            process::exit(1);
        }
    }
}

fn parse_file(input_path: &Path) {
    let input = fs::read_to_string(input_path).expect("Failed to read input file");

    match tokenize(&input) {
        Ok(tokens) => {
            match parse(tokens) {
                Ok(ast) => {
                    println!("Parsing completed. AST:");
                    println!("{}", ast);
                }
                Err(e) => {
                    eprintln!("Parsing error: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Lexing error: {}", e);
            process::exit(1);
        }
    }
}

fn compile(input_path: &Path) -> PathBuf {
    let preprocessed_file = preprocess(input_path);
    let assembly_file = generate_assembly_file(&preprocessed_file);
    fs::remove_file(preprocessed_file).expect("Failed to remove preprocessed file");
    let executable = assemble_and_link(&assembly_file);
    fs::remove_file(assembly_file).expect("Failed to remove assembly file");
    executable
}

fn preprocess(input_path: &Path) -> PathBuf {
    let preprocessed_file = input_path.with_extension("i");
    run_command("gcc", &["-E", "-P", input_path.to_str().unwrap(), "-o", preprocessed_file.to_str().unwrap()]);
    preprocessed_file
}

fn generate_assembly_file(input_path: &Path) -> PathBuf {
    let assembly_file = input_path.with_extension("s");
    fs::write(&assembly_file, "# Stubbed assembly output").expect("Failed to write assembly file");
    assembly_file
}

fn assemble_and_link(assembly_file: &Path) -> PathBuf {
    let output_file = assembly_file.with_extension("");
    run_command("gcc", &[assembly_file.to_str().unwrap(), "-o", output_file.to_str().unwrap()]);
    output_file
}

fn run_command(command: &str, args: &[&str]) {
    let status = process::Command::new(command)
        .args(args)
        .status()
        .unwrap_or_else(|_| panic!("Failed to execute {}", command));

    if !status.success() {
        eprintln!("Command execution failed: {} {:?}", command, args);
        process::exit(1);
    }
}