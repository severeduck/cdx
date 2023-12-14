use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Keyword,
    Identifier,
    Literal,
    Operator,
    Number,
}

// Token struct
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

// Lexer
pub struct Lexer {
    source_code: String,
    token_patterns: HashMap<TokenType, Regex>,
}

impl Lexer {
    // Create a new Lexer instance
    pub fn new(source_code: String) -> Self {
        let mut token_patterns = HashMap::new();

        // Define regex patterns for each token type
        token_patterns.insert(TokenType::Keyword, Regex::new(r"\b(function|if|else)\b").unwrap());
        token_patterns.insert(TokenType::Identifier, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());
        token_patterns.insert(TokenType::Literal, Regex::new(r"\b\d+\b").unwrap()); // Simple integer literals
        token_patterns.insert(TokenType::Operator, Regex::new(r"[+*/-]").unwrap());

        Lexer {
            source_code,
            token_patterns,
        }
    }

    // Tokenize the source code
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        // Iterate over token patterns and match them in the source code
        for (token_type, pattern) in &self.token_patterns {
            for cap in pattern.captures_iter(&self.source_code) {
                tokens.push(Token {
                    token_type: token_type.clone(),
                    value: cap[0].to_string(),
                });
            }
        }

        // Sort tokens by their position in the source code
        tokens.sort_by_key(|token| self.source_code.find(&token.value).unwrap());

        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // Logic to move to the next token
        // This is a simplistic example; you'll need to adapt it to your lexer's logic
        // For example:
        if let Some(token) = self.token_patterns.iter().find_map(|(type, pattern)| {
            pattern.find(&self.source_code).map(|mat| Token {
                token_type: type.clone(),
               value: mat.as_str().to_string(),
           })
        }) {
            self.source_code = self.source_code[token.value.len()..].to_string(); // Update the remaining source code
               Some(token)
        } else {
            None // No more tokens
        }
    }
}

fn main() {
    let source_code = String::from("function add(a, b) if a > b a else b");
    let lexer = Lexer::new(source_code);

    let tokens = lexer.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_simple() {
        let source = "function test() {}";
        let lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        assert!(!tokens.is_empty(), "Lexer should produce tokens");
    }
}
