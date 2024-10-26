from concurrent.futures import ThreadPoolExecutor
import multiprocessing
from typing import List, Tuple, Callable
from preprocessor import preprocess_folder
from compiler import (run_lexer, run_parser, run_llvm_ir_generator, run_x86_64_generator, run_full_compiler)
from test_runner import run_tests, TestResult
from reporter import print_results, print_summary
from utils import cleanup_samples_directory
from threading import Lock

print_lock = Lock()

def run_stage(stage_info: Tuple[int, str, Callable, List[str], List[str]]) -> Tuple[int, str, List[TestResult]]:
    stage_order, stage_name, test_func, invalid_files, valid_files = stage_info
    stage_results = run_tests(test_func, invalid_files, valid_files)
    return (stage_order, stage_name, stage_results)

def main():
    num_cores = multiprocessing.cpu_count()
    num_threads = num_cores * 2
    print(f"Running tests using {num_threads} threads ({num_cores} CPU cores)")
    print("-" * 80)

    with ThreadPoolExecutor(max_workers=num_threads) as executor:
        preprocess_futures = [
            executor.submit(preprocess_folder, "samples/valid"),
            executor.submit(preprocess_folder, "samples/invalid_lex"),
            executor.submit(preprocess_folder, "samples/invalid_parse")
        ]
        
    valid_preprocessed = preprocess_futures[0].result()
    invalid_lex_preprocessed = preprocess_futures[1].result()
    invalid_parse_preprocessed = preprocess_futures[2].result()

    test_stages = [
        (1, "Lexer", run_lexer, invalid_lex_preprocessed, invalid_parse_preprocessed + valid_preprocessed),
        (2, "Parser", run_parser, invalid_parse_preprocessed, valid_preprocessed),
        (3, "LLVM IR Generation", run_llvm_ir_generator, [], valid_preprocessed),
        (4, "Assembly Generation", run_x86_64_generator, [], valid_preprocessed),
        (5, "Full Compilation", run_full_compiler, [], valid_preprocessed),
    ]

    stage_results = []
    with ThreadPoolExecutor(max_workers=min(len(test_stages), num_threads)) as executor:
        futures = [executor.submit(run_stage, stage_info) for stage_info in test_stages]
        for future in futures:
            try:
                stage_order, stage_name, results = future.result()
                with print_lock:
                    print(f"\n{stage_name} Test Results:")
                    print("-" * 80)
                    passed = print_results(results)
                    stage_results.append((stage_order, f"{stage_name} Tests", passed))
            except Exception as e:
                print(f"Error running stage: {e}")

    with print_lock:
        print("\nCleaning up samples directory...")
        files_removed = cleanup_samples_directory()
        print(f"Cleanup completed. Removed {files_removed} non-'.c' files")
        print_summary(sorted(stage_results, key=lambda x: x[0]))

if __name__ == "__main__":
    main()