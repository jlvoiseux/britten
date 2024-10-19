import subprocess
import os
from typing import List, Union
from os import PathLike

def run_lexer(file_path):
    command = [r"..\target\debug\britten.exe", "--lex", file_path]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return 0
    except subprocess.CalledProcessError as e:
        if e.returncode == 1:
            return 1
        else:
            print(f"Error running britten on {file_path}: {e}")
            return None

def test_lexer():
    invalid_file_paths = get_files_in_folder("invalid_lex")
    valid_file_paths = get_files_in_folder("valid")
    return run_tests(run_lexer, invalid_file_paths, valid_file_paths)

def run_parser(file_path):
    command = [r"..\target\debug\britten.exe", "--parse", file_path]
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        return 0
    except subprocess.CalledProcessError as e:
        if e.returncode == 1:
            return 1
        else:
            print(f"Error running britten on {file_path}: {e}")
            return None

def test_parser():
    invalid_file_paths = get_files_in_folder("invalid_parse")
    valid_file_paths = get_files_in_folder("valid")
    return run_tests(run_parser, invalid_file_paths, valid_file_paths)

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

def print_results(results):
    print("\nTest Results:")
    print("-" * 80)
    print(f"{'File Path':<40} {'Result':<10} {'Expected':<10} {'Success':<10}")
    print("-" * 80)
    for file_path, result, expected, success in results:
        print(f"{file_path:<40} {str(result):<10} {str(expected):<10} {str(success):<10}")

    total_tests = len(results)
    successful_tests = sum(1 for _, _, _, success in results if success)
    print("-" * 80)
    print(f"Total tests: {total_tests}")
    print(f"Successful tests: {successful_tests}")
    print(f"Failed tests: {total_tests - successful_tests}")

    return successful_tests == total_tests

def print_summary(lexer_passed, parser_passed):
    print("\nTEST SUMMARY:")
    print(f"LEXER TESTS {'PASSED' if lexer_passed else 'FAILED'}")
    print(f"PARSER TESTS {'PASSED' if parser_passed else 'FAILED'}")

if __name__ == "__main__":
    lexer_results = test_lexer()
    print("Lexer Test Results:")
    lexer_passed = print_results(lexer_results)

    parser_results = test_parser()
    print("\nParser Test Results:")
    parser_passed = print_results(parser_results)

    print_summary(lexer_passed, parser_passed)