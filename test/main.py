from preprocessor import preprocess_files
from compiler import (run_lexer, run_parser, run_llvm_ir_generator, run_x86_64_generator, run_full_compiler)
from test_runner import run_tests
from reporter import print_stage_header, print_results, print_summary
from utils import cleanup_samples_directory

def main():
    print_stage_header("Preprocessing")
    valid_preprocessed = preprocess_files("samples/valid")
    invalid_lex_preprocessed = preprocess_files("samples/invalid_lex")
    invalid_parse_preprocessed = preprocess_files("samples/invalid_parse")

    test_stages = [
        ("Lexer Tests", run_lexer, invalid_lex_preprocessed, invalid_parse_preprocessed + valid_preprocessed),
        ("Parser Tests", run_parser, invalid_parse_preprocessed, valid_preprocessed),
        ("LLVM IR Generation Tests", run_llvm_ir_generator, [], valid_preprocessed),
        ("Assembly Generation Tests", run_x86_64_generator, [], valid_preprocessed),
        ("Full Compilation Tests", run_full_compiler, [], valid_preprocessed),
    ]

    results = []
    for stage_name, test_func, invalid_files, valid_files in test_stages:
        print_stage_header(stage_name)
        stage_results = run_tests(test_func, invalid_files, valid_files)
        passed = print_results(stage_results)
        results.append((stage_name, passed))

    print("\nCleaning up samples directory...")
    files_removed = cleanup_samples_directory()
    print(f"Cleanup completed. Removed {files_removed} non-'.c' files")

    print_summary(results)

if __name__ == "__main__":
    main()