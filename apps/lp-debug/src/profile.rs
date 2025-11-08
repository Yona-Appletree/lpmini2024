/// Profile the demo program using pprof
use engine_core::test_engine::demo_program::run_demo_with_profiling;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let width = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(16);
    
    let height = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(16);
    
    let num_frames = args.get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    
    let output_path = args.get(4)
        .map(|s| s.as_str())
        .unwrap_or("flamegraph.svg");

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

