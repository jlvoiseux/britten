# Britten
A C compiler written in Rust. 
- Implements both the frontend and the backend, with the LLVM IR as its intermediate representation
- A test bench is included and is run in [Github actions](https://github.com/jlvoiseux/britten/actions)

Note: this is a work in progress and currently only supports compilation of int-only programs with unary operators, binary operators, and direct return values.

![britten](https://github.com/user-attachments/assets/16ae4662-cef8-4ec7-ab08-858c4967c6c4)
<img width="1422" alt="Screenshot 2025-02-09 153558" src="https://github.com/user-attachments/assets/ec84b6c6-0564-4d6a-9d89-34869b7deabe" />

## Prerequisites
- Python3 (for the test bench)
- Install Clang and lldb (used for preprocessing, linking, and for the test bench)

## Usage
*Tested on WSL2 only*
Preprocessing, executable generation from assembly code and linking are omitted for now. To compile a c file, do the following:
```sh
clang -E -P file.c -o file.i
./britten file.i
clang file.s -o file
```

### Running tests
On the linux distribution of your choice:
```sh
cargo build --release --target x86_64-unknown-linux-gnu
python3 test/test.py
```

## References
- [*Writing a C compiler**](https://nostarch.com/writing-c-compiler) from Nora Sandler
- Individual test cases come from the MIT-licensed [companion test suite](https://github.com/nlsandler/writing-a-c-compiler-tests) of the aforementioned book.
