# Pseudocompiler
![](https://github.com/teymour-aldridge/pseudocompiler/workflows/Build/badge.svg)
![](https://github.com/teymour-aldridge/pseudocompiler/workflows/Tests/badge.svg)
[![Run on Repl.it](https://repl.it/badge/github/teymour-aldridge/pseudocompiler)](https://repl.it/github/teymour-aldridge/pseudocompiler)

A compiler for pseudocode, inspired by the [OCR specification](https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf) [[backup link](https://web.archive.org/web/20200118155656/https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf)].

# Getting started
At present, you will have to either use the web frontend or build the compiler from source. Please note that the compiler is way off of being finished so it doesn't do much (or anything) at the moment.

# Contributing
If you'd like to contribute there are a few things which would be useful:
* Adding more tests to the compiler (in the `src` directory, under the `tests` subdirectory).
* The [recursive descent parser](https://en.wikipedia.org/wiki/Recursive_descent_parser) is currently a bit of a mess and could do with some being worked upon.
* There are two languages we want to compile to. These are LLVM IR and Javascript. Help working upon this would be useful.
* Building the frontend (in React). This is still being worked upon.

Please note that we adopt the Contributor Covenant Code of Conduct. 

# Code structure
This repository serves as a monorepo. It is laid out something like this:
```
- app
- src
```
The `app` directory contains the code for a web frontend for the compiler which uses React (and WebAssembly to build the compiler for the web).
The `src` directory contains the code for the actual compiler, written in Rust.

## Compiler code organisation
The compiler's code is organised something like this:
```
src/
 - parser
 - transpiler
 - tests
```
The `parser` directory stored both the lexer and the parser (which are in the files `lexer.rs` and `parser.rs` respectively). They also include useful enums and structs. 

The `transpiler` directory stores the code for the transpilation to other languages (LLVM and JS). Currently no attempts have been made to translate to LLVM. Translation to JS is a little bit further in the works, but still a way of being complete.

## Frontend code organisation
The web interface is a React app (created initially with `create-react-app`). Components sit in their own folders (everything is grouped around them, rather than being separated into different folders for view, state, CSS styling, etc). 

# Language specification
This still hasn't completely been worked out. 

# CI
We use Github actions for continuous integration (deploying the frontend to Github Pages) and testing. The Github actions we use are configured in `.github/workflows`. 