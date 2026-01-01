use std::{env, fs, process};

mod parser;
mod runtime;
mod lexer;

fn main() {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the file path argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path_or_code>", args[0]);
        process::exit(1);
    }

    // read code
    let file_path_or_code = &args[1];
    let code = if file_path_or_code.ends_with(".bf") {
        // Read the file content
        match fs::read_to_string(file_path_or_code) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file '{}': {}", file_path_or_code, err);
                process::exit(1);
            }
        }
    } else {
        // Use passed arg as code
        file_path_or_code.to_owned()
    };

    // remove non code chars
    let code = code
        .replace("\n", "")
        .replace("\r", "")
        .replace("\t", "")
        .replace(" ", "");

    // lexing
    let lexer_result = lexer::lex(&code);
    let tokens = match lexer_result {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Error during tokenizing: {:?}", err.message());
            process::exit(1);
        }
    };

    // parsing
    let parser_result = parser::parse(&tokens);
    let ast = match parser_result {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Error during parsing: {:?}", err.message());
            process::exit(1);
        }
    };

    // executing
    let execution_result = runtime::execute(ast);
    match execution_result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error during execution: {:?}", err.message());
            process::exit(1);
        }
    }
}
