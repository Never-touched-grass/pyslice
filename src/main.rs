mod confparse;
mod generator;

use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <lexer_file.lex> [output.py]", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() >= 3 { &args[2] } else { "lexer.py" };

    // 1. Parse the .lex file
    let tokens = match confparse::parse(input_path) {
        Ok(toks) => toks,
        Err(e) => {
            eprintln!("Error parsing {}: {}", input_path, e);
            std::process::exit(1);
        }
    };

    // 2. Generate Python lexer code
    let py_code = generator::generate(tokens);

    // 3. Write to output file
    match File::create(output_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(py_code.as_bytes()) {
                eprintln!("Error writing to {}: {}", output_path, e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Cannot create {}: {}", output_path, e);
            std::process::exit(1);
        }
    }

    println!("Generated Python lexer at {}", output_path);
}
