/// Quick benchmark for test scene
use engine_core::test_scene::{render_test_scene, SceneData};
use engine_core::test_engine::fixed_from_f32;
use std::time::Instant;

const FRAME_COUNT: u32 = 1000;

fn main() {
    println!("Test Engine Benchmark (host)");
    println!("Running {} frames...\n", FRAME_COUNT);

    let mut scene = SceneData::new();
    let start = Instant::now();

    for i in 0..FRAME_COUNT {
        let time = fixed_from_f32(i as f32 * 0.01);
        render_test_scene(&mut scene, time);
    }

    let elapsed = start.elapsed();
    let total_us = elapsed.as_micros() as u64;
    let avg_us = total_us / FRAME_COUNT as u64;
    let fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };

    println!("Total: {:?}", elapsed);
    println!("Average: {}µs/frame", avg_us);
    println!("FPS: {}", fps);
}
