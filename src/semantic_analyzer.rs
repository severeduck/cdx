use crate::ast::{AstNode};
use crate::lexer::{Token};
use crate::error::{YourErrorType};


pub struct SemanticAnalyzer {
    // You can add fields here if needed for context or state
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            // Initialize any fields here
        }
    }

    pub fn analyze(&self, ast: &AstNode) -> Result<(), String> {
        // Starting point for AST traversal and analysis
        match ast {
            AstNode::NumberLiteral(value) => {
                // Check if the value is an integer
                if value.fract() == 0.0 {
                    Ok(())
                } else {
                    Err(format!("Error: Non-integer number found at line {}", ast.line_number))
                }
            },
            // Add more matches for different AST node types
            // ...
            _ => Ok(()), // Default case if no specific handling is required
        }
    }

    // You can add more helper methods here for specific checks or AST node types
}
