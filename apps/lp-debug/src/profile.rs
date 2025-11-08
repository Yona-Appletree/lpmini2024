/// Profile the demo program using pprof
use engine_core::test_engine::demo_program::run_demo_with_profiling;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let width = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(150);

    let height = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(150);

    let num_frames = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(300);

    // Create profiling_data directory if it doesn't exist
    let output_dir = Path::new("profiling_data");
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).expect("Failed to create profiling_data directory");
    }

    // Determine output path - always place in profiling_data directory
    let output_path_buf = if let Some(filename) = args.get(4) {
        // If custom filename provided, place it in profiling_data directory
        output_dir.join(filename)
    } else {
        // Default filename
        output_dir.join("profile.svg")
    };
    let output_path = output_path_buf.to_str().unwrap();

    println!("Profiling demo program:");
    println!("  Resolution: {}x{}", width, height);
    println!("  Frames: {}", num_frames);
    println!("  Output: {}", output_path);
    println!();

    match run_demo_with_profiling(width, height, num_frames, output_path) {
        Ok(()) => println!("Profiling completed successfully"),
        Err(e) => {
            eprintln!("Error during profiling: {}", e);
            std::process::exit(1);
        }
    }
}
