use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "lp-pool-to-alloc")]
#[command(about = "Migrate codebase from lp-pool to lp-alloc")]
struct Args {
    /// Dry run - don't write changes
    #[arg(long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Paths to process (directories or files)
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    for path in &args.paths {
        if path.is_dir() {
            process_directory(path, &args);
        } else if path.is_file() {
            process_file(path, &args);
        } else {
            eprintln!("Warning: {} does not exist", path.display());
        }
    }
}

fn process_directory(dir: &Path, args: &Args) {
    for entry in WalkDir::new(dir).into_iter().filter_entry(|e| {
        let path = e.path();
        // Skip target directories and hidden files
        !path.to_string_lossy().contains("/target/")
            && !path.to_string_lossy().contains("/.git/")
            && path.extension().map_or(false, |ext| ext == "rs")
    }) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                process_file(entry.path(), args);
            }
        }
    }
}

fn process_file(file_path: &Path, args: &Args) {
    if args.verbose {
        println!("Processing: {}", file_path.display());
    }

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", file_path.display(), e);
            return;
        }
    };

    // Parse and transform the file
    let transformed = lp_pool_to_alloc::transform_file(&content, file_path);

    if transformed != content {
        if args.verbose {
            println!("  -> File modified");
        }

        if !args.dry_run {
            if let Err(e) = fs::write(file_path, transformed) {
                eprintln!("Error writing {}: {}", file_path.display(), e);
            }
        } else {
            println!("  -> Would modify (dry run)");
        }
    }
}
