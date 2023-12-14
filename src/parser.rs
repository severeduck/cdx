
use crate::lexer::{Lexer, Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
        };
        parser.next_token(); // Initialize the first token
        parser
    }

    pub fn next_token(&mut self) {
        // Move to the next token
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        // Assuming the top-level structure of your language is an expression
        // or a series of expressions. You may need to adjust this logic based
        // on your language's syntax and grammar.

        let mut ast_nodes = Vec::new();

        while self.current_token.is_some() {
            let node = self.parse_expression()?;
            ast_nodes.push(node);

            // Depending on your language, you might have a delimiter like ';' 
            // between statements or expressions at the top level.
            // If so, consume it here. If not, remove this part.
            if matches!(self.current_token, Some(Token { token_type: TokenType::Semicolon, .. })) {
                self.next_token();
            }
        }

        // For simplicity, let's assume the top-level structure is a single node.
        // If your language allows multiple top-level nodes (like multiple statements or expressions),
        // you might want to return a different kind of AstNode (e.g., AstNode::Program) that contains all nodes.
        ast_nodes.into_iter().next().ok_or_else(|| "No valid expressions found".to_string())
    }

    fn parse_lambda_expression(&mut self) -> Result<AstNode, String> {
        self.expect_token(TokenType::Pipe)?; // Assuming '|' is used to denote the start of lambda parameters
        let parameters = self.parse_parameters()?; // You will need to implement this method
        self.expect_token(TokenType::Pipe)?; // Assuming '|' is used to denote the end of lambda parameters
        let body = self.parse_expression()?; // You will need to implement this method
        Ok(AstNode::LambdaExpression { parameters, body }) // Return a new AST node for the lambda
    }

    // Helper method to expect and consume a specific token type, returning an error if it's not found
    fn expect_token(&mut self, token_type: TokenType) -> Result<(), String> {
        match self.current_token {
            Some(ref token) if token.type == token_type => {
                self.next_token(); // Consume the token
                Ok(())
            },
            _ => Err(format!("Expected token: {:?}", token_type)),
        }
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, String> {
        let mut parameters = Vec::new();
        while let Some(token) = &self.current_token {
            match token.type {
                TokenType::Identifier => {
                    let param_name = token.value.clone();
                    self.next_token(); // Consume the identifier
                    let type_annotation = if matches!(self.current_token, Some(Token { type: TokenType::Colon, .. })) {
                        self.next_token(); // Consume the colon
                        Some(self.parse_type()?) // Parse the type
                    } else {
                        None
                    };
                    parameters.push(Parameter { name: param_name, type_annotation });
                    if matches!(self.current_token, Some(Token { type: TokenType::Comma, .. })) {
                        self.next_token(); // Consume the comma and continue
                    } else {
                        break; // End of parameters
                    }
                },
                _ => return Err(format!("Expected parameter, found {:?}", token.type)),
            }
        }
        Ok(parameters)
    }

    fn parse_expression(&mut self) -> Result<AstNode, String> {
        let mut lhs = self.parse_primary()?;
        while let Some(token) = &self.current_token {
            match token.type {
                TokenType::Plus | TokenType::Minus => {
                    let operator = token.type;
                    self.next_token(); // Consume the operator
                    let rhs = self.parse_primary()?;
                    lhs = AstNode::BinaryOperation {
                        lhs: Box::new(lhs),
                        operator,
                        rhs: Box::new(rhs),
                    };
                },
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<AstNode, String> {
        match &self.current_token {
            Some(Token { type: TokenType::Number, value }) => {
                self.next_token(); // Consume the number token
                Ok(AstNode::NumberLiteral(value.parse::<f64>().unwrap()))
            },
            Some(Token { type: TokenType::Identifier, value }) => {
                self.next_token(); // Consume the identifier token
                Ok(AstNode::Variable(value.clone()))
            },
            // Add more cases here as needed (e.g., parentheses, function calls)
            _ => Err("Unrecognized primary expression".to_string()),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        // Parsing type annotations
        // For example, parsing a type identifier like 'int', 'string', etc.
        match &self.current_token {
            Some(Token { type: TokenType::Identifier, value }) => {
                // Map the value to a Type enum variant
                let type_enum = match value.as_str() {
                    "int" => Type::Int,
                    "string" => Type::String,
                    // ... other type mappings ...
                    _ => return Err(format!("Unknown type: {}", value)),
                };
                self.next_token(); // Consume the type token
                Ok(type_enum)
            }
            _ => Err("Expected a type".to_string()),
        }
    }
}

#[derive(Debug)] // Add this line
enum AstNode {
    LambdaExpression {
        parameters: Vec<Parameter>,
        body: Box<AstNode>,
    },
    NumberLiteral(f64),
    Variable(String),
}

// Assume a `Parameter` struct is defined elsewhere
struct Parameter {
    name: String,
    type_annotation: Option<Type>,
    // ... any additional fields ...
}

enum Type {
    Int,
    String,
}




fn main() {
    // Example usage of the parser
    let source_code = "your source code here";
    let lexer = Lexer::new(source_code.to_string());
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(e) => println!("Error parsing: {}", e),
    }
}
