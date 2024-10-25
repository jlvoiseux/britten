from typing import Callable, List, Tuple, Optional

TestResult = Tuple[str, Optional[int], int, bool]

def run_tests(test_function: Callable[[str], Optional[int]], 
              invalid_files: List[str], 
              valid_files: List[str]) -> List[TestResult]:
    results = []

    def process_files(file_paths: List[str], expected_result: int) -> None:
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