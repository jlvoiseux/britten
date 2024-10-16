import subprocess
import os

def run_britten(file_path):
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
    results = []

    invalid_file_paths = [
        r"lexer\invalid\at_sign.c",
        r"lexer\invalid\backslash.c",
        r"lexer\invalid\backtick.c",
        r"lexer\invalid\invalid_identifier.c",
        r"lexer\invalid\invalid_identifier_2.c",
    ]

    for file_path in invalid_file_paths:
        result = run_britten(file_path)
        if result is not None:
            success = result == 1
            results.append((file_path, result, 1, success))
        else:
            results.append((file_path, None, 1, False))

    valid_file_paths = [
        r"lexer\valid\multi_digit.c",
        r"lexer\valid\newlines.c",
        r"lexer\valid\no_newlines.c",
        r"lexer\valid\return_0.c",
        r"lexer\valid\return_2.c",
        r"lexer\valid\spaces.c",
        r"lexer\valid\tabs.c",
    ]

    for file_path in valid_file_paths:
        result = run_britten(file_path)
        if result is not None:
            success = result == 0
            results.append((file_path, result, 0, success))
        else:
            results.append((file_path, None, 1, False))

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

if __name__ == "__main__":
    results = test_lexer()
    print_results(results)