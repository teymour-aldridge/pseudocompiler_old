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

# Language specification
This still hasn't completely been worked out. 

# CI
We use Github actions for continuous integration (deploying the frontend to Github Pages) and testing. The Github actions we use are configured in `.github/workflows`. 