use std::fmt;

use anyhow::Result;

use crate::lexer::lexer::{Lexer, Token};

// represents nodes of AST
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(u8),
    Bool(bool),
    Add(Box<ASTNode>, Box<ASTNode>),
    Multiply(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
}

// result evaluations are either int or bool
#[derive(Debug, PartialEq)]
pub enum ResultEval {
    Int(u8),
    Bool(bool),
}

// allowing instances of ResultEval to be formatted as strings
impl fmt::Display for ResultEval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResultEval::Int(value) => write!(f, "{:?}", value),
            ResultEval::Bool(value) => write!(f, "{:?}", value),
        }
    }
}

pub struct ShuntiyardParser {
    lexer: Lexer,
    operator_stack: Vec<Token>,
    output_queue: Vec<ASTNode>,
}
impl ShuntiyardParser {
    // initializing shuntiyard parser with the provided lexer
    pub fn new(lexer: Lexer) -> ShuntiyardParser {
        let parser = ShuntiyardParser {
            lexer: lexer,
            operator_stack: Vec::new(),
            output_queue: Vec::new(),
        };
        return parser;
    }

    // checks if left or right side of multiplication is 0 to simply evaluation
    pub fn check_for_zero(&self, l_node: ASTNode, r_node: ASTNode) -> ASTNode {
        if (l_node == ASTNode::Number(0)) | (r_node == ASTNode::Number(0)) {
            //println!("One side of tree is zero");
            return ASTNode::Number(0);
        } else {
            return ASTNode::Multiply(Box::new(l_node), Box::new(r_node));
        }
    }

    // pops last two nodes from output_queue and performs an operation based on the provided operator
    pub fn add_node(&mut self, operator: &Token) {
        let l_node = self.output_queue.pop().unwrap();
        let r_node = self.output_queue.pop().unwrap();

        let node = match operator {
            Token::Add(_) => ASTNode::Add(Box::new(l_node), Box::new(r_node)),
            Token::Mult(_) => self.check_for_zero(l_node, r_node),
            Token::Or(_) => ASTNode::Or(Box::new(r_node), Box::new(l_node)),
            _ => unimplemented!("Operator not defined"),
        };
        self.output_queue.push(node);
    }

    // parsing of the input tokens using the Shunting Yard algorithm until EOF
    pub fn parse(&mut self) -> Result<ASTNode> {
        while let Ok(token) = self.lexer.next_token() {
            match token {
                // Converting Zero, One, True, False to ASTNode & push to output_queue 
                Token::Zero | Token::One => self
                    .output_queue
                    .push(ASTNode::Number(token.to_string().parse().unwrap())),
                Token::True => self.output_queue.push(ASTNode::Bool(true)),
                Token::False => self.output_queue.push(ASTNode::Bool(false)),
                // Add, Mult, Or: checks the topmost operator on the operator_stack 
                // and compares its precedence with the current operator
                Token::Add(o1) | Token::Mult(o1) | Token::Or(o1) => {
                    while self.operator_stack.len() > 0 && self.operator_stack.last() != None {
                        match self.operator_stack.last() {
                            Some(Token::Add(o2)) | Some(Token::Mult(o2)) | Some(Token::Or(o2)) => {
                                if o1 <= *o2 {
                                    let op = self.operator_stack.pop().unwrap();
                                    self.add_node(&op);
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                    self.operator_stack.push(token)
                }
                // left parenthesis pushed to operator stack
                Token::LPar => self.operator_stack.push(token),
                // right parenthesis
                Token::RPar => loop {
                    match self.operator_stack.last() {
                        // pops operators from the operator_stack until LPar found
                        Some(&Token::LPar) => {
                            self.operator_stack.pop().unwrap();
                            break;
                        }
                        // add_node to create the corresponding AST nodes
                        _ => {
                            let op = &self.operator_stack.pop().unwrap();
                            self.add_node(op);
                        }
                    }
                },
                Token::Eof => break,
            }
            // println!(
            //     "Current Token {:?} & Current Stack {:?} & Current output queue {:?}",
            //     token, self.operator_stack, self.output_queue
            // )
        }
        while self.operator_stack.len() > 0 {
            // Pop them off and push them to the output_queue
            let op = &self.operator_stack.pop().unwrap();
            self.add_node(op);
        }
        // println!(
        //     "End Stack {:?} & End output queue {:?}",
        //     self.operator_stack, self.output_queue
        // );
        Ok(self.output_queue.pop().unwrap())
    }
}

// #[cfg(test)]
// mod test {
//     use crate::lexer::lexer::Lexer;
//     use crate::parser::parser::{ResultEval, ShuntiyardParser};
//     use ::anyhow::Result;

//     #[test]
//     fn parsing_test() -> Result<()> {
//         Ok(())
//     }
// }
