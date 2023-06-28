# RUST evaluator
This project is part of the ModelBasedSW course.
<br>The evaluator translates a simple language of arithmetic expressions (consisting of Integer and Boolean expressions) and returns their result. Based on a context-free grammar, the evaluator simplifies zero product properties like 0*x = 0. 


## Prerequisites
- [rust](https://www.rust-lang.org/tools/install)


## Setup
- install rust
  <br>_(installation on windows may require the installation of the visual studio installer)_
- install rust dependencies with
    
    `>> cargo build`



## Project structure
This project consists of a lexer, a parser, a simplifier and the evaluator. 


### Grammar
This evaluator is based on the following context-free grammar:
```
E -> 1
E -> 0
E -> True
E -> False
E -> E + E
E -> E * E
E -> E || E
E -> (E)
```


### Usage
The evaluator can be used with<br>
    `>> cargo run main.rs "1 * 0 + 1"`


### Lexer
The lexer turns the input string into a stream of tokens. 
<br>The following characters are assigned to token types: 
```
    "1" => One
    "0" => Zero
    "(" => LPar
    ")" => RPar
    "*" => Mult() 
    "+" => Add() 
    "||" => Or() 
    "true" => True
    "false" => False
    End of file => Eof
```
Whitespaces are skipped. Other characters are unallowed. 


### Parser
This project uses the Shunting Yard algorithm to handle operator precedence and ensure that the AST reflects the correct order of operations. The method reads tokens from the lexer until the end of the input (Token::Eof) is reached.


### Simplifier
* Simplification of zero product properties like 0*x = 0
* Simplification of each node until input AST is same as result


### Evaluator
Evaluates an abstract syntax tree (AST) node and returns the result. Short-circuit evaluation is applied for OR nodes. Integer and Boolean expressions cannot be mixed.


### Tests
Each of the evaluation steps provides at least one unittest. The tests can be executed with<br>

   `>> cargo test -- --nocapture`


## Comparison between Haskell and Rust

### Overview

| Aspect                      | Rust                                             | Haskell                                                |
|-----------------------------|--------------------------------------------------|--------------------------------------------------------|
| Data Types                  | Enums (`enum`)                                   | Algebraic Data Types (`data`)                           |
|                             | Allows associated data                           | Supports sum types and product types                    |
|                             | Box required (Otherwise recursive definition)    | Automatic garbage collection, no need for pointers   |
| Pattern Matching            | `match` expression                               | `case` expression                                      |
|                             | Expects exhaustive handling of cases             | Expects exhaustive handling of cases                    |
|                             | Destructuring of enum variants                    | Destructuring of algebraic data types                   |
| Memory Management           | Ownership and borrowing system                    | Immutable data structures                              |
| Error Handling              | `Result` and `Option` types                       | Monads (`Maybe`, `Either`) or custom monads             |
|                             | Use of `anyhow` or `thiserror` crates             | Precise and composable error handling                   |
| Immutability                | Default immutability, overidden by `mut` keyword | Strict immutability                                    |
|                             | N/A                                              | Transformations with immutable data structures          |
| Functional Programming      | N/A                                              | Leveraging recursion and higher-order functions         |

### Implementation of Add and Multiplication

**Haskell:**
<br>type checking for both values + calculation (addition/multiplication) can be performed in one step via pattern matching
```
eval (Plus e1 e2) =
    let r1 = eval e1
        r2 = eval e2
    in case (r1, r2) of
        (Just (Left i1), Just (Left i2)) -> Just (Left (i1 + i2))
        (_, _) -> Nothing
```

**Rust:**
<br>explicit checks and option handling are necessary to ensure that the types are correct before calculation takes place
```
ASTNode::Add(left, right) => {
    let l1 = eval(left);
    let r1 = eval(right);
    if !l1.is_some() && !r1.is_some() {
        return None;
    } else {
        let val_l1 = match l1.unwrap() {
            ResultEval::Int(value) => value,
            _ => return None,
        };
        let val_l2 = match r1.unwrap() {
            ResultEval::Int(value) => value,
            _ => return None,
        };
        Some(ResultEval::Int(val_l1 + val_l2))
    }
}
```