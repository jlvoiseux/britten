import subprocess
import os
from typing import List, Optional
from compiler import compile_with_clang
from utils import run_subprocess
from concurrent.futures import ThreadPoolExecutor

def run_preprocessor(input_file: str, is_valid: bool) -> Optional[str]:
    output_file = input_file.replace('.c', '.i')
    
    success, error = run_subprocess(['clang', '-E', '-P', input_file, '-o', output_file])
    if not success:
        if is_valid and error:
            print(f"Error preprocessing valid file:\n{error}")
        return None
        
    if is_valid:
        clang_exe = input_file.replace('.c', '.clang')
        compile_with_clang(input_file, clang_exe)
            
    return output_file

def preprocess_folder(folder_name: str) -> List[str]:
    folder_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder_name)
    c_files = [f for f in os.listdir(folder_path) if f.endswith('.c')]
    
    is_valid = "valid" in folder_path and "invalid" not in folder_path
    
    preprocessed = []
    with ThreadPoolExecutor() as executor:
        futures = [
            executor.submit(run_preprocessor, os.path.join(folder_path, c_file), is_valid)
            for c_file in c_files
        ]
        for future in futures:
            result = future.result()
            if result:
                preprocessed.append(result)
            
    return preprocessed