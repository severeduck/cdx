mod lexer;
mod parser;

fn main() {
    let source_code = "your source code here"; // Replace this with actual source code input

    let lexer = lexer::Lexer::new(source_code.to_string());
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(e) => println!("Error parsing: {}", e),
    }
}
