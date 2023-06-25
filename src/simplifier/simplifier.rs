use crate::parser::parser::ASTNode;

pub fn simplify(ast: &ASTNode) -> ASTNode {
    match ast {
        // num & bool: returns same node
        ASTNode::Number(0) => ASTNode::Number(0),
        ASTNode::Number(1) => ASTNode::Number(1),
        ASTNode::Bool(true) => ASTNode::Bool(true),
        ASTNode::Bool(false) => ASTNode::Bool(false),
        // Add: recursively simplifies the left and right children
        ASTNode::Add(left, right) => {
            ASTNode::Add(Box::new(simplify(left)), Box::new(simplify(right)))
        }
        // Mutiplication: checks both nodes for zero values, returns 0 if found
        ASTNode::Multiply(left, right) => {
            if (**left == ASTNode::Number(0)) | (**right == ASTNode::Number(0)) {
                // println!("right: {:?}, left: {:?}", **right, **left);
                ASTNode::Number(0)
            } else {
                ASTNode::Multiply(Box::new(simplify(left)), Box::new(simplify(right)))
            }
        }
        // Or: recursively simplifies the left and right children
        ASTNode::Or(left, right) => {
            ASTNode::Or(Box::new(simplify(left)), Box::new(simplify(right)))
        }
        _ => unreachable!(),
    }
}
// application of "simplify" until return input AST as the final result
pub fn simplify_fix(ast: ASTNode) -> ASTNode {
    let ast2 = simplify(&ast);
    if ast2 == ast {
        return ast;
    } else {
        simplify_fix(ast2)
    }
}

#[cfg(test)]
mod test {
    use anyhow::{Ok, Result};

    use crate::{parser::parser::ASTNode, simplifier::simplifier::simplify_fix};
    #[test]
    fn simplify_fix_mult_zero_test() -> Result<()> {
        let ast1 = ASTNode::Multiply(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Add(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(1)),
            )),
        );
        let exp_ast = ASTNode::Number(0);
        let simp_ast = simplify_fix(ast1);
        println!(
            "Expected AST: {:?}, Simplified AST: {:?}",
            exp_ast, simp_ast
        );
        assert_eq!(simp_ast, exp_ast);
        Ok(())
    }
    #[test]
    fn simplify_fix_test() -> Result<()> {
        let ast1 = ASTNode::Multiply(
            Box::new(ASTNode::Number(1)),
            Box::new(ASTNode::Add(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(1)),
            )),
        );
        let exp_ast = ASTNode::Multiply(
            Box::new(ASTNode::Number(1)),
            Box::new(ASTNode::Add(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(1)),
            )),
        );
        let simp_ast = simplify_fix(ast1);
        println!(
            "Expected AST: {:?}, Simplified AST: {:?}",
            exp_ast, simp_ast
        );
        assert_eq!(exp_ast, simp_ast);
        Ok(())
    }
    #[test]
    fn simplify_or_test() -> Result<()> {
        let ast1 = ASTNode::Multiply(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Bool(false)),
            )),
        );
        let exp_ast = ASTNode::Multiply(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Bool(false)),
            )),
        );
        let simp_ast = simplify_fix(ast1);
        println!(
            "Expected AST: {:?}, Simplified AST: {:?}",
            exp_ast, simp_ast
        );
        println!(
            "\x1b[33mits expected that the test fails, \
            because the simplification does not simplifies ill-types expressions\x1b[0m"
        );
        assert_eq!(exp_ast, simp_ast);
        Ok(())
    }
}
