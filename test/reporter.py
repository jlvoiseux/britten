from typing import List, Tuple, Optional
from utils import get_parent_and_filename

TestResult = Tuple[str, Optional[int], int, bool]

def print_stage_header(stage_name: str) -> None:
    print(f"\nRunning {stage_name}...")
    print("-" * 80)

def print_results(results: List[TestResult]) -> bool:
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

def print_summary(test_results: List[Tuple[str, bool]]) -> None:
    print("\nTEST SUMMARY:")
    print("-" * 80)
    all_passed = True
    
    for stage, passed in test_results:
        status = "PASSED" if passed else "FAILED"
        print(f"{stage:<30} {status}")
        all_passed = all_passed and passed
    
    print("-" * 80)
    print(f"OVERALL STATUS: {'PASSED' if all_passed else 'FAILED'}")