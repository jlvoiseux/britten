from concurrent.futures import ThreadPoolExecutor
from typing import Callable, List, Tuple, Optional
import multiprocessing
from queue import Queue
from threading import Lock

TestResult = Tuple[str, Optional[int], int, bool]

def run_test(args: Tuple[str, int, Callable]) -> TestResult:
    file_path, expected_result, test_function = args
    result = test_function(file_path)
    if result is not None:
        success = result == expected_result
        return (file_path, result, expected_result, success)
    return (file_path, None, expected_result, False)

def run_tests(test_function: Callable[[str], Optional[int]], invalid_files: List[str], valid_files: List[str]) -> List[TestResult]:
    test_args = [(f, 1, test_function) for f in invalid_files]
    test_args.extend((f, 0, test_function) for f in valid_files)
    
    num_threads = min(len(test_args), multiprocessing.cpu_count() * 2)
    results = []
    
    with ThreadPoolExecutor(max_workers=num_threads) as executor:
        futures = []
        for args in test_args:
            futures.append((args[0], executor.submit(run_test, args)))
            
        for file_path, future in futures:
            try:
                result = future.result()
                results.append(result)
            except Exception as e:
                print(f"Error processing {file_path}: {e}")
                results.append((file_path, None, 1 if file_path in invalid_files else 0, False))
                
    return sorted(results, key=lambda x: x[0])