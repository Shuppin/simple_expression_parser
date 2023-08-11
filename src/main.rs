mod tokeniser;
mod parser;
mod ast;

use std::io::{self, Write};

use parser::Parser;

fn main() {
    
    let mut parser = Parser::new(
        String::new()
    );
    
    loop {
        print!("> ");
        io::stdout().flush()
            .expect("Failed to flush stdout");
        
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            parser.set_source(input);

            match parser.parse() {
                Ok(tree) => {
                    println!("\n{}\n", tree.display(0));
                    println!("answer = {}\n", tree.evaluate());
                },
                Err(msg) => {
                    println!("Failed to parse: {}", msg);
                }
            };
        } else {
            println!("Failed to read stdin");
        }

    }

}
