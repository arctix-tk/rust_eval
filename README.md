# RUST evaluator
This project is part of the ModelBasedSW course.
<br>The evaluator translates a simple language of arithmetic expressions (consisting of Integer and Boolean expressions) and returns their result. Based on a context-free grammar, the evaluator simplifies zero product properties like 0*x = 0. 

## prerequisites:
- [rust](https://www.rust-lang.org/tools/install)

## setup:
- install rust
  <br>_(installation on windows may require the installation of the visual studio installer)_
- install rust dependencies with
    
    `>> cargo build`

## usage:
### context free grammar
This evaluator is based on the following context-free grammar:
E -> 1
E -> 0
E -> True
E -> False
E -> E + E
E -> E * E
E -> E || E
E -> (E)
### 

## comparison between Haskell and Rust: