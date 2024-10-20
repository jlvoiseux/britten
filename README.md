## Britten
A C compiler written in Rust. 
- Implements both the frontend and the backend, with the LLVM IR as its intermediate representation
- A test bench is included

### Prerequisites
- Python3 (for the test bench)
- Install Clang and lldb (used for preprocessing, linking, and for the test bench)

### Usage
Preprocessing, executable generation from assembly code and linking are omitted for now. To compile a c file, do the following:
```sh
clang -E -P file.c -o file.i
./britten file.i
clang file.s -o file
```

### Running tests
On the linux distribution of your choice:
```sh
cargo build --target x86_64-unknown-linux-gnu
python3 test/test.py
```

### References
- [*Writing a C compiler**](https://nostarch.com/writing-c-compiler) from Nora Sandler
- The majority of test cases come from the [companion test suite](https://github.com/nlsandler/writing-a-c-compiler-tests) of the aforementioned book.