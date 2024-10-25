import subprocess
import os
from typing import List, Tuple, Optional
from config import BRITTEN_PATH
from utils import run_subprocess, get_executable_return_code

CompilerResult = Tuple[str, Optional[int], int, bool]

def compile_with_clang(input_file: str, output_file: str) -> Optional[str]:
    success, error = run_subprocess(['clang', input_file, '-o', output_file])
    if not success:
        if error:
            print(f"Error compiling valid file with clang:\n{error}")
        return None
    return output_file

def run_lexer(file_path: str) -> Optional[int]:
    success, error = run_subprocess([BRITTEN_PATH, "--lex", file_path])
    is_invalid = "invalid_lex" in file_path
    if not success and not is_invalid and error:
        print(error)
    return 1 if not success else 0

def run_parser(file_path: str) -> Optional[int]:
    success, error = run_subprocess([BRITTEN_PATH, "--parse", file_path])
    is_invalid = "invalid_parse" in file_path or "invalid_lex" in file_path
    if not success and not is_invalid and error:
        print(error)
    return 1 if not success else 0

def run_llvm_ir_generator(file_path: str) -> Optional[int]:
    if "invalid" in file_path:
        return 1
        
    britten_output = file_path.replace('.i', '.ll')
    britten_exe = file_path.replace('.i', '.britten.llvm')
    clang_exe = file_path.replace('.i', '.clang')
    
    if not os.path.exists(clang_exe):
        print(f"No reference executable found: {file_path}")
        return None
        
    success, error = run_subprocess([BRITTEN_PATH, "--llvm", file_path])
    if not success:
        print(error)
        return 1
            
    if not os.path.exists(britten_output):
        print(f"Britten failed to generate LLVM IR for {file_path}")
        return 1
    
    # Compile britten-generated LLVM using clang
    success, error = run_subprocess(['clang', britten_output, '-o', britten_exe])
    if not success:
        print(error)
        return 1

    clang_code = get_executable_return_code(clang_exe)
    britten_code = get_executable_return_code(britten_exe)
    
    if clang_code is None or britten_code is None:
        return None
        
    return 0 if clang_code == britten_code else 1

def run_x86_64_generator(file_path: str) -> Optional[int]:
    if "invalid" in file_path:
        return 1
        
    success, error = run_subprocess([BRITTEN_PATH, "--codegen", file_path])
    if not success:
        print(error)
    return 1 if not success else 0

def run_full_compiler(file_path: str) -> Optional[int]:
    if "invalid" in file_path:
        return 1
        
    britten_exe = file_path.replace('.i', '.britten')
    clang_exe = file_path.replace('.i', '.clang')
    assembly_file = file_path.replace('.i', '.s')
    
    if not os.path.exists(clang_exe):
        print(f"No reference executable found: {file_path}")
        return None
        
    success, error = run_subprocess([BRITTEN_PATH, file_path])
    if not success:
        print(error)
        return 1

    if not os.path.exists(assembly_file):
        print(f"No assembly file generated for {file_path}")
        return 1
    
    # Assemble and link britten-generated x86_64 asm using clang
    success, error = run_subprocess(['clang', assembly_file, '-o', britten_exe])
    if not success:
        print(error)
        return 1
    
    clang_code = get_executable_return_code(clang_exe)
    britten_code = get_executable_return_code(britten_exe)
    
    if clang_code is None or britten_code is None:
        return None

    if clang_code != britten_code:
        print(f"Compilation error for {file_path}: Britten-compiled exe returned {britten_code}, while clang-compiled exe returned {clang_code}")
        return 1
    
    return 0