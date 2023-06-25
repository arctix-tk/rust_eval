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

### Used grammar
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
The evaluator can be used with following command
    `>> cargo run main.rs ""`

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
This project uses the Shunting Yard algorithm to handle operator precedence and ensure that the AST reflects the correct order of operations. AST multiplication nodes are checked for 0 values. The method reads tokens from the lexer until the end of the input (Token::Eof) is reached.

### Simplifier
* Simplification of zero product properties like 0*x = 0
* Simplification of each node until input AST is same as result

### Evaluator
Evaluates an abstract syntax tree (AST) node and returns the result. Short-circuit evaluation is applied for OR nodes.

### Tests
Each of the evaluation steps provides at least one unittest. The tests can be executed with
    `>> cargo test -- --nocapture`



## Comparison between Haskell and Rust
