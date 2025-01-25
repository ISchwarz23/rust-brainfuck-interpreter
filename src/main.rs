use std::{env, fs, process};

mod parser;
mod runtime;
mod tokenizer;

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

    // Clean code
    let code = code.replace("\n", "")
        .replace("\r", "")
        .replace("\t", "")
        .replace(" ", "");

    // Interpret code
    let tokens = tokenizer::tokenize(&code)
        .map_err(|e| e.message())
        .expect("Error during tokenizing");
    let expressions = parser::parse(&tokens)
        .map_err(|e| e.message())
        .expect("Error during parsing");
    runtime::execute(expressions)
        .map_err(|e| e.message())
        .expect("Error during execution");
}
