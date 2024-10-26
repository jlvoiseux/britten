import os
import subprocess
from typing import List, Union, Optional, Tuple
from os import PathLike

def format_command_error(cmd: List[str], e: subprocess.CalledProcessError) -> str:
    lines = [
        f"Command failed with exit code {e.returncode}:",
        f"Command: {' '.join(cmd)}",
    ]
    if e.stdout:
        lines.append("stdout:")
        lines.extend(f"  {line}" for line in e.stdout.decode().splitlines())
    if e.stderr:
        lines.append("stderr:")
        lines.extend(f"  {line}" for line in e.stderr.decode().splitlines())
    return "\n".join(lines)

def run_subprocess(cmd: List[str], check: bool = True) -> Tuple[bool, Optional[str]]:
    try:
        result = subprocess.run(cmd, check=check, capture_output=True, text=False)
        return True, None
    except subprocess.CalledProcessError as e:
        error_msg = format_command_error(cmd, e)
        return False, error_msg

def get_files_in_folder(folder_name: Union[str, PathLike]) -> List[str]:
    folder_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder_name)
    return [
        os.path.join(folder_name, f)
        for f in os.listdir(folder_path)
        if os.path.isfile(os.path.join(folder_path, f))
    ]

def get_parent_and_filename(absolute_path: str) -> str:
    parent = os.path.basename(os.path.dirname(absolute_path))
    filename = os.path.basename(absolute_path)
    return f"{parent}/{filename}"

def cleanup_samples_directory() -> int:
    samples_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'samples')
    files_removed = 0
    for root, dirs, files in os.walk(samples_dir):
        for file in files:
            if not file.endswith('.c') and file != 'LICENSE':
                file_path = os.path.join(root, file)
                try:
                    os.remove(file_path)
                    files_removed += 1
                except Exception as e:
                    print(f"Error removing file {file_path}: {e}")
    return files_removed

def get_executable_return_code(executable: str) -> Optional[int]:
    try:
        result = subprocess.run([executable], capture_output=True, text=False)
        return result.returncode
    except Exception as e:
        print(f"Error executing {executable}: {e}")
        return None