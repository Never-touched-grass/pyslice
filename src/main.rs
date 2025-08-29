mod confparse;
mod generator;

use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} [options(build, --test)] <lexer_file.lex> [output.py] [test_input]\nDo pyslice --help for more info.",
            args[0]
        );
        std::process::exit(1);
    }

    let opt = &args[1];
    if args.len() == 2 && opt != "--help" {
        let input_path = &args[1];
        println!("Starting REPL...");
        println!("Type 'exit' to quit.");
        let mut inp = String::new();

        loop {
            inp.clear();
            print!("# ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut inp).expect("Failed to read line.");

            let inp_trim = inp.trim();
            if inp_trim == "exit" { break; }
            if inp_trim.is_empty() { continue; }
            let tokens = match confparse::parse(input_path) {
                Ok(toks) => toks,
                Err(e) => {
                    eprintln!("Error parsing {}: {}", input_path, e);
                    continue;
                }
            };

            let mut py_code = generator::generate(tokens);
            let verbose_flag = "False"; 
            py_code.push_str(&format!(
                "\nlexer = Lexer('{}', verbose={})\nfor token in lexer.tokenize():\n    print(token)\n",
                inp_trim, verbose_flag
            ));
            std::fs::write("temp.py", py_code).expect("Failed to write temp.py");
            #[cfg(target_os = "windows")]
            let py_cmd = "py";
            #[cfg(not(target_os = "windows"))]
            let py_cmd = "python3";
            let status = Command::new(py_cmd)
                .arg("temp.py")
                .status()
                .expect("Failed to run Python");

            if !status.success() {
                eprintln!("Python execution failed");
            }

            let _ = std::fs::remove_file("temp.py"); 
        }

        std::process::exit(0);
    }

    if opt == "--test" {
        if args.len() < 4 {
            eprintln!("Usage: pyslice --test <lexer_file.lex> \"some input text\" [--verbose]");
            std::process::exit(1);
        }

        let input_path = &args[2];
        let input_stream = &args[3];
        let verbose = args.get(4).map_or(false, |v| v == "--verbose");
        let verbose_flag = if verbose { "True" } else { "False" };

        let tokens = match confparse::parse(input_path) {
            Ok(toks) => toks,
            Err(e) => {
                eprintln!("Error parsing {}: {}", input_path, e);
                std::process::exit(1);
            }
        };

        let mut py_code = generator::generate(tokens);
        py_code.push_str(&format!(
            "\nlexer = Lexer('{}', verbose={})\nfor token in lexer.tokenize():\n    print(token)\n",
            input_stream, verbose_flag
        ));

        std::fs::write("temp.py", py_code).expect("Failed to write temp.py");

        #[cfg(target_os = "windows")]
        let py_cmd = "py";
        #[cfg(not(target_os = "windows"))]
        let py_cmd = "python3";

        let status = Command::new(py_cmd)
            .arg("temp.py")
            .status()
            .expect("Failed to run Python");

        if !status.success() {
            eprintln!("Python execution failed");
        }
        #[cfg(target_os = "windows")]
        let _ = Command::new("cmd")
            .args(&["/C", "del", "temp.py"])
            .status();
        #[cfg(not(target_os = "windows"))]
        let _ = Command::new("rm")
            .args(&["-rf", "temp.py"])
            .status();

        std::process::exit(0);
    } else if opt == "--help" {
        println!("Available commands:");
        println!("pyslice build: build a Python lexer from a .lex file.");
        println!("Usage: pyslice build <lexer_filename.lex> <output_filename.py(leave blank for lexer.py, put - for stdout)>");
        println!("pyslice --test: create a temporary Python lexer and print tokens, then delete it.");
        println!("Optional: --verbose: show matched lexemes in addition to tokens.");
        std::process::exit(0);
    } else if opt == "build" {
        let input_path = &args[2];
        let output_path = if args.len() >= 4 { &args[3] } else { "lexer.py" };

        let tokens = match confparse::parse(input_path) {
            Ok(toks) => toks,
            Err(e) => {
                eprintln!("Error parsing {}: {}", input_path, e);
                std::process::exit(1);
            }
        };

        let py_code = generator::generate(tokens);

        if output_path == "-" {
            println!("{}", py_code);
        } else {
            std::fs::write(output_path, py_code).expect("Failed to write output file");
            println!("Generated Python lexer at {}", output_path);
        }

        std::process::exit(0);
    }
}
