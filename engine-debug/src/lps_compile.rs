use lp_script::compile_script;
use engine_debug::lpa_format::program_to_lpa;
/// LPS to LPA compiler CLI
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let (input_source, output_file) = match args.len() {
        1 => {
            // No args: read from stdin, write to stdout
            ("-".to_string(), None)
        }
        2 => {
            // One arg: input file, write to stdout
            (args[1].clone(), None)
        }
        3 => {
            // Two args: input file, output file
            (args[1].clone(), Some(args[2].clone()))
        }
        _ => {
            eprintln!("Usage: lps-compile [input.lps] [output.lpa]");
            eprintln!("  No args: read from stdin, write to stdout");
            eprintln!("  One arg: read from file, write to stdout");
            eprintln!("  Two args: read from input file, write to output file");
            std::process::exit(1);
        }
    };

    // Read input
    let source = if input_source == "-" {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    } else {
        fs::read_to_string(&input_source).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", input_source, e);
            std::process::exit(1);
        })
    };

    // Compile
    let program = compile_script(&source).unwrap_or_else(|e| {
        eprintln!("Compile error: {}", e);
        std::process::exit(1);
    });

    // Generate LPA
    let lpa = program_to_lpa(&program);

    // Write output
    if let Some(output_path) = output_file {
        fs::write(&output_path, lpa).unwrap_or_else(|e| {
            eprintln!("Error writing {}: {}", output_path, e);
            std::process::exit(1);
        });
        eprintln!("Compiled {} -> {}", input_source, output_path);
    } else {
        print!("{}", lpa);
    }
}

