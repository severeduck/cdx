
mod lexer;
mod parser;
mod semantic_analyzer;
mod codegen;

fn main() {
    let source_code = "your source code here"; // Replace this with actual source code input

    // Lexer initialization
    let lexer = lexer::Lexer::new(source_code.to_string());

    // Parser initialization
    let mut parser = parser::Parser::new(lexer);

    // Parsing process
    match parser.parse() {
        Ok(ast) => {
            // Semantic Analysis
            let analyzer = semantic_analyzer::SemanticAnalyzer::new();
            match analyzer.analyze(&ast) {
                Ok(_) => {
                    // Code Generation
                    let codegen = codegen::CodeGenerator::new();
                    match codegen.generate(&ast) {
                        Ok(code) => println!("Generated Code: {}", code),
                        Err(e) => println!("Error in code generation: {}", e),
                    }
                },
                Err(e) => println!("Semantic Analysis Error: {}", e),
            }
        },
        Err(e) => println!("Parsing Error: {}", e),
    }
}
