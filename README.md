# Lox Interpreter in Rust

## Overview

This project is an implementation of the Lox programming language interpreter, built in Rust. Lox is a dynamically typed scripting language with features like variables, control flow, functions, and object-oriented programming.

## Features

### Completed

- **Lexer and Parser:** Implements a lexer and recursive descent parser for Lox.
- **Expression Evaluation:** Supports arithmetic expressions and precedence handling (e.g., `2 + 2 * 2` will be `2 + (2 * 2)` which evaluates to `6`).
- **Print Statements:** Handles `print` statements to output results (e.g., `print 2 + 2;` outputs `4`).
- **Variable Storage:** Supports variable declarations and usage (e.g., `var i = 2; print i;` outputs `2`).

### In Progress
- **Scope**: It defines a region where a name maps to a certain entity, and will potentially help us get `classes` and `functions`  
For Example: 
        ```python
        {
            var a = "first block",
            print a; // "first block"
        }

        {
            var a = "second block",
            print a; // second block
        }
        ```
### Future
- **Control Flow:** Planned support for `if`, `else`, `while`, and `for` loops.
- **Functions:** Planned support for defining and calling functions.
- **Classes and Objects:** Planned support for class definitions, instantiation, and method calls.
- **Closures:** Planned support for closures and lexical scoping.

