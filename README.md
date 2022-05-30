# expr.rs
 A safe and simple math evaluator for Rust.

## Features
Basic operations such as 

- addition
- subtraction
- multiplication
- division
- modulo
- exponentiation
- negation
- factorial

are supported

Features like sin, cos, tan, log, and more are planned for the future.

A python interface is also planned for the future.

## Usage
```rust
use expr_rs::parser;

assert_eq!(parser::eval("1 + 2").unwrap().to_string(), "3");
```

In Python

```py
import expr_rs

assert expr_rs.eval("1 + 2") == 3
```

In CLI

```bash
cargo install expr_rs

expr 1 + 2
```
