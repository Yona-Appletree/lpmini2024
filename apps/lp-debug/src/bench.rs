use std::time::Instant;

/// Quick benchmark for test scene at multiple resolutions
use engine_core::test_engine::demo_program::create_demo_scene;
use engine_core::test_engine::scene::SceneRuntime;
use engine_core::test_engine::RuntimeOptions;
use lp_script::fixed::ToFixed;

const FRAME_COUNT: u32 = 1000;

fn benchmark_size(width: usize, height: usize) {
    // Create scene with demo program
    let config = create_demo_scene(width, height);
    let options = RuntimeOptions::new(width, height);
    let mut scene = SceneRuntime::new(config, options).expect("Valid scene");

    let start = Instant::now();

    for i in 0..FRAME_COUNT {
        let time = (i as f32 * 0.01).to_fixed();
        scene.render(time, 1).expect("Render failed");
    }

    let elapsed = start.elapsed();
    let total_us = elapsed.as_micros() as u64;
    let avg_us = total_us / FRAME_COUNT as u64;
    let fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };

    println!("{}x{}: {}us/frame ({} FPS)", width, height, avg_us, fps);
}

fn main() {
    println!("Test Engine Benchmark (host)");
    println!(
        "Running {} frames at multiple resolutions...\n",
        FRAME_COUNT
    );

    benchmark_size(8, 8);
    benchmark_size(12, 12);
    benchmark_size(16, 16);
    benchmark_size(20, 20);
    benchmark_size(24, 24);
}
