import subprocess
import os
from typing import List, Union
from os import PathLike

BRITTEN_PATH = r"/../target/x86_64-unknown-linux-gnu/debug/britten"

def run_preprocessor(input_file):
    output_file = input_file.replace('.c', '.i')
    command = ['clang', '-E', '-P', input_file, '-o', output_file]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return output_file
    except subprocess.CalledProcessError as e:
        print(f"Error preprocessing {input_file}: {e}")
        return None

def preprocess_files(folder_name):
    folder_path = os.path.abspath(os.path.join(os.path.dirname(__file__), folder_name))
    c_files = [f for f in os.listdir(folder_path) if f.endswith('.c')]
    preprocessed_files = []
    for c_file in c_files:
        full_path = os.path.join(folder_path, c_file)
        preprocessed_file = run_preprocessor(full_path)
        if preprocessed_file:
            preprocessed_files.append(preprocessed_file)
    return preprocessed_files

def run_assembler_linker(assembly_file):
    output_file = assembly_file.replace('.s', '')
    command = ['clang', assembly_file, '-o', output_file]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return output_file
    except subprocess.CalledProcessError as e:
        print(f"Error assembling and linking {assembly_file}: {e}")
        return None

def assemble_and_link_files(folder_name):
    folder_path = os.path.abspath(os.path.join(os.path.dirname(__file__), folder_name))
    assembly_files = [f for f in os.listdir(folder_path) if f.endswith('.s')]
    executables = []
    for assembly_file in assembly_files:
        full_path = os.path.join(folder_path, assembly_file)
        executable = run_assembler_linker(full_path)
        if executable:
            executables.append(executable)
    return executables

def run_lexer(file_path):
    command = [os.path.dirname(os.path.abspath(__file__)) + BRITTEN_PATH, "--lex", file_path]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return 0
    except subprocess.CalledProcessError as e:
        if e.returncode == 1:
            return 1
        else:
            print(f"Error running britten on {file_path}: {e}")
            return None

def run_parser(file_path):
    command = [os.path.dirname(os.path.abspath(__file__)) + BRITTEN_PATH, "--parse", file_path]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return 0
    except subprocess.CalledProcessError as e:
        if e.returncode == 1:
            return 1
        else:
            print(f"Error running britten on {file_path}: {e}")
            return None

def run_generator(file_path):
    command = [os.path.dirname(os.path.abspath(__file__)) + BRITTEN_PATH, "--codegen", file_path]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return 0
    except subprocess.CalledProcessError as e:
        if e.returncode == 1:
            return 1
        else:
            print(f"Error running britten codegen on {file_path}: {e}")
            return None

def run_compiler(file_path):
    command = [os.path.dirname(os.path.abspath(__file__)) + BRITTEN_PATH, file_path]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return 0
    except subprocess.CalledProcessError as e:
        if e.returncode == 1:
            return 1
        else:
            print(f"Error running britten codegen on {file_path}: {e}")
            return None

def run_executables(executables, expected_exit_codes):
    results = []
    for exe in executables:
        exe_name = os.path.basename(exe)
        expected_code = expected_exit_codes.get(exe_name, 0)  # Default to 0 if not specified
        try:
            result = subprocess.run(exe, check=False, capture_output=True, text=True)
            actual_code = result.returncode
            success = actual_code == expected_code
            results.append((exe_name, actual_code, expected_code, success))
        except Exception as e:
            print(f"Error running {exe}: {e}")
            results.append((exe_name, None, expected_code, False))
    return results

def get_files_in_folder(folder_name: Union[str, PathLike]) -> List[str]:
    folder_path = os.path.abspath(os.path.join(os.path.dirname(__file__), folder_name))
    return [
        os.path.join(folder_name, f)
        for f in os.listdir(folder_path)
        if os.path.isfile(os.path.join(folder_path, f))
    ]

def run_tests(test_function, invalid_files, valid_files):
    results = []

    def process_files(file_paths, expected_result):
        for file_path in file_paths:
            result = test_function(file_path)
            if result is not None:
                success = result == expected_result
                results.append((file_path, result, expected_result, success))
            else:
                results.append((file_path, None, expected_result, False))

    process_files(invalid_files, 1)
    process_files(valid_files, 0)

    return results

def cleanup_samples_directory():
    samples_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), 'samples'))
    files_removed = 0
    for root, dirs, files in os.walk(samples_dir):
        for file in files:
            if not file.endswith('.c'):
                file_path = os.path.join(root, file)
                try:
                    os.remove(file_path)
                    files_removed += 1
                except Exception as e:
                    print(f"Error removing file {file_path}: {e}")
    
    print(f"\nCleanup completed. Removed {files_removed} non-'.c' files from the samples directory.")
    
def get_parent_and_filename(absolute_path):
    parent = os.path.basename(os.path.dirname(absolute_path))
    filename = os.path.basename(absolute_path)
    return f"{parent}/{filename}"

def print_results(results):
    print("\nTest Results:")
    print("-" * 80)
    print(f"{'File Path':<40} {'Result':<10} {'Expected':<10} {'Success':<10}")
    print("-" * 80)
    for file_path, result, expected, success in results:
        print(f"{get_parent_and_filename(file_path):<40} {str(result):<10} {str(expected):<10} {str(success):<10}")

    total_tests = len(results)
    successful_tests = sum(1 for _, _, _, success in results if success)
    print("-" * 80)
    print(f"Total tests: {total_tests}")
    print(f"Successful tests: {successful_tests}")
    print(f"Failed tests: {total_tests - successful_tests}")

    return successful_tests == total_tests

def print_executable_results(results):
    print("\nExecutable Run Results:")
    print("-" * 80)
    print(f"{'Executable':<30} {'Actual Exit Code':<20} {'Expected Exit Code':<20} {'Success':<10}")
    print("-" * 80)
    for exe_name, actual_code, expected_code, success in results:
        print(f"{exe_name:<30} {str(actual_code):<20} {str(expected_code):<20} {str(success):<10}")

    total_runs = len(results)
    successful_runs = sum(1 for _, _, _, success in results if success)
    print("-" * 80)
    print(f"Total runs: {total_runs}")
    print(f"Successful runs: {successful_runs}")
    print(f"Failed runs: {total_runs - successful_runs}")

    return successful_runs == total_runs

def print_summary(lexer_passed, parser_passed, codegen_passed, compilation_passed, executables_passed):
    print("\nTEST SUMMARY:")
    print(f"LEXER TESTS {'PASSED' if lexer_passed else 'FAILED'}")
    print(f"PARSER TESTS {'PASSED' if parser_passed else 'FAILED'}")
    print(f"CODE GENERATION TESTS {'PASSED' if codegen_passed else 'FAILED'}")
    print(f"FULL COMPILATION TESTS {'PASSED' if compilation_passed else 'FAILED'}")
    print(f"EXECUTABLE RUNS {'PASSED' if executables_passed else 'FAILED'}")

if __name__ == "__main__":
    print("Preprocessing files...")
    valid_preprocessed = preprocess_files("samples/valid")
    invalid_lex_preprocessed = preprocess_files("samples/invalid_lex")
    invalid_parse_preprocessed = preprocess_files("samples/invalid_parse")

    print("\nRunning Lexer Tests...")
    lexer_results = run_tests(run_lexer, invalid_lex_preprocessed, valid_preprocessed)
    print("Lexer Test Results:")
    lexer_passed = print_results(lexer_results)

    print("\nRunning Parser Tests...")
    parser_results = run_tests(run_parser, invalid_parse_preprocessed, valid_preprocessed)
    print("Parser Tests Results:")
    parser_passed = print_results(parser_results)

    print("\nRunning Code Generation tests...")
    generator_results = run_tests(run_generator, [], valid_preprocessed)
    print("Code Generation Tests Results:")
    generator_passed = print_results(generator_results)

    print("\nRunning Full Compilation Tests...")
    compilation_results = run_tests(run_compiler, [], valid_preprocessed)
    print("Full Compilation Tests Results:")
    compilation_passed = print_results(compilation_results)

    print("\nAssembling and Linking...")
    executables = assemble_and_link_files("samples/valid")
    print(f"Generated executables: {executables}")

    expected_exit_codes = {
        "multi_digit": 100,
        "newlines": 0,
        "no_newlines": 0,
        "return_2": 2,
        "return_0": 0,
        "spaces": 0,
        "tabs": 0,
    }

    print("\nRunning executables and checking exit codes...")
    executable_results = run_executables(executables, expected_exit_codes)
    executables_passed = print_executable_results(executable_results)

    print_summary(lexer_passed, parser_passed, generator_passed, compilation_passed, executables_passed)

    print("\nCleaning up samples directory...")
    cleanup_samples_directory()