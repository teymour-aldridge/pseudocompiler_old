# Pseudocompiler
A compiler for pseudocode, inspired by the [OCR specification](https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf) [[backup link](https://web.archive.org/web/20200118155656/https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf)].

# Code structure
This repository serves as a monorepo. It is laid out something like this:
```
- app
- src
```
The `app` directory contains the code for a web frontend for the compiler which uses React (and WebAssembly to build the compiler for the web).
The `src` directory contains the code for the actual compiler, written in Rust.

# Language specification
This still hasn't completely been worked out. 

# Contributing
If you'd like to contribute there are a few things which would be useful:
* Adding more tests to the compiler (in the `src` directory, under the `tests` subdirectory).
* Building the frontend (in React). This is still being worked upon.

Please note that we adopt the Contributor Covenant Code of Conduct. 