
# CDX Compiler

## Overview
Welcome to the CDX Compiler! 
This is a Rust-based compiler with an exciting journey ahead - we're planning to bootstrap a compiler for CDX in CDX itself. Here's a sneak peek into our roadmap.

## Bootstrapping a CDX Compiler in CDX
The idea of writing a compiler for CDX in CDX is thrilling and challenging. We're laying down the groundwork for this ambitious project, and here's how we plan to approach it:

### 1. Designing the Compiler Architecture
- We're planning a lexer that can tokenize CDX code beautifully, a parser that's sharp enough to build an AST, a semantic analyzer to keep our code in check, and a code generator that's the final piece of our puzzle.

### 2. Implementing the Compiler Components
- Our journey starts with simple functionality and will gradually climb to handle the complexities of CDX.

### 3. Writing Unit Tests
- Testing is at the heart of our development. We're building a robust suite of unit tests to ensure every component works flawlessly.

### 4. Updating Documentation
- We believe in clear documentation. As we build our compiler, we'll document every step to guide you and future contributors.

### 5. Iterative Development and Testing
- It's all about taking small steps and testing rigorously. We'll be refining our compiler with regular testing.

### 6. Validation and Optimization
- Our goal is a compiler that not only adheres to CDX specifications but also excels in performance and robustness.

## Getting Started

### Prerequisites

Before you get started, you'll need:

- The Rust programming language.
- Cargo, which is Rust's package manager.

### Installation

Here's how you can get up and running:

1. Clone our repository:

   ```bash
   git clone https://github.com/severeduck/cdx.git
   ```

2. Jump into the project directory:

   ```bash
   cd cdx_compiler
   ```

3. Let's build the project:

   ```bash
   cargo build --release
   ```

   This step compiles everything and gets it ready for action.

## Usage

Ready to use the CDX Compiler?
Simply run the compiled binary with the necessary arguments.
Check out the documentation or examples in the project for more detailed instructions on getting started.

### Building from Source

If you're interested in building the compiler straight from the source, here's how:

```bash
cargo build --release
```

Your fresh executable will be waiting for you in `target/release`.

### Running Tests

Want to run our test suite? Just execute:

```bash
cargo test
```

It's always good to make sure everything is ticking along nicely!

## Development

If you're keen to dive into development, the `src` directory is your go-to place to understand the nuts and bolts of the CDX Compiler.
You'll find additional guidelines and standards for development in our project documentation.

## Contributing
We love contributions! 
If you're interested in contributing to the CDX Compiler, please take a look at the contributing guidelines for more information on how to get involved.

## License
This project is all about open source and is licensed under the MIT License. 
Feel free to use, modify, and distribute it as you see fit.

## Acknowledgments
A huge shoutout to all the contributors and supporters of the CDX Compiler project. 
We couldn't do it without you!
