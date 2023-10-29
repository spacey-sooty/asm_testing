Assembly Testing
================

This is a repository where I am testing out Assembly and using it in other languages.
This repository has 3 languages in it; C, Assembly and Rust.
The C code merely calls the Assembly functions and displays the Result.
The Assembly is linked to it in the `Makefile`.
The Rust code does all the linking itself in the `build.rs` file.
The Assembly is our some simple Assembly functions I have written.

Simple Commands
---------------

`make compile`  
This compiles all of the code; Assembly, Rust and C.

`make run`  
This runs all of the code.

`make clean`  
This cleans the output directories; `build` and `target`.

`make test`  
This tests the code.
Currently only the Rust code is tested.

