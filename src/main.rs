use std::env;
use std::process::{exit, Command};
use std::path::{Path, PathBuf};
use std::fs;

mod lexer;
use lexer::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [--lex|--parse|--codegen|-S] <input_file>", args[0]);
        exit(1);
    }

    let (option, input_file) = args.get(1).and_then(|arg| {
        if arg.starts_with("--") || arg == "-S" {
            Some((Some(arg.as_str()), args.get(2)?))
        } else {
            Some((None, arg))
        }
    }).unwrap_or_else(|| {
        eprintln!("Invalid arguments");
        exit(1);
    });

    let input_path = Path::new(input_file);
    if !input_path.exists() {
        eprintln!("Input file does not exist: {}", input_file);
        exit(1);
    }

    match option {
        Some("--lex") => {
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
                    exit(1);
                }
            }
        },
        Some("--parse") => println!("Parsing completed"),
        Some("--codegen") => println!("Code generation completed"),
        Some("-S") => {
            let assembly_file = generate_assembly_file(input_path);
            println!("Assembly file generated: {}", assembly_file.display());
        },
        None => {
            let executable = compile(input_path);
            println!("Executable generated: {}", executable.display());
        },
        _ => {
            eprintln!("Unknown option: {:?}", option);
            exit(1);
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
    run_command("gcc", &["-E", "-P", input_path.to_str().unwrap(), "-o", preprocessed_file.to_str().unwrap()], "Preprocessing");
    preprocessed_file
}

fn generate_assembly_file(input_path: &Path) -> PathBuf {
    let assembly_file = input_path.with_extension("s");
    fs::write(&assembly_file, "# Stubbed assembly output").expect("Failed to write assembly file");
    assembly_file
}

fn assemble_and_link(assembly_file: &Path) -> PathBuf {
    let output_file = assembly_file.with_extension("");
    run_command("gcc", &[assembly_file.to_str().unwrap(), "-o", output_file.to_str().unwrap()], "Assembly and linking");
    output_file
}

fn run_command(command: &str, args: &[&str], step: &str) {
    let status = Command::new(command)
        .args(args)
        .status()
        .unwrap_or_else(|_| panic!("Failed to execute {}", command));

    if !status.success() {
        eprintln!("{} failed", step);
        exit(1);
    }
}