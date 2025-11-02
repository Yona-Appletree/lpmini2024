use std::cmp::Ord;
use perf_tests_common;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;
const BUFFER_SIZE: usize = WIDTH * HEIGHT * 3;

fn print_buffer(name: &str, buffer: &[u8]) {
    println!("\n{}:", name);
    for y in 0..HEIGHT {
        print!("  ");
        for x in 0..WIDTH {
            let idx = (y * WIDTH + x) * 3;
            print!("{:3} ", buffer[idx]);
        }
        println!();
    }
}

fn compare_buffers(name1: &str, buf1: &[u8], name2: &str, buf2: &[u8]) -> bool {
    const TOLERANCE: u8 = 10; // Allow +/- 10 units difference
    
    let mut max_diff = 0u8;
    let mut total_diff = 0u32;
    let mut diff_count = 0;
    let mut exceeds_tolerance = 0;
    
    for i in 0..buf1.len() {
        let diff = (buf1[i] as i16 - buf2[i] as i16).abs() as u8;
        if diff > 0 {
            max_diff = max_diff.max(diff);
            total_diff += diff as u32;
            diff_count += 1;
            if diff > TOLERANCE {
                exceeds_tolerance += 1;
            }
        }
    }
    
    let pass = exceeds_tolerance == 0;
    
    if diff_count > 0 {
        println!("\n{} vs {}:", name1, name2);
        println!("  Pixels different: {}/{}", diff_count, buf1.len());
        println!("  Max difference: {}", max_diff);
        println!("  Avg difference: {:.2}", total_diff as f32 / diff_count as f32);
        println!("  Exceeds tolerance (>{}): {}", TOLERANCE, exceeds_tolerance);
        if pass {
            println!("  ✓ PASS (within tolerance)");
        } else {
            println!("  ✗ FAIL (too different)");
        }
    } else {
        println!("\n{} vs {}: IDENTICAL ✓", name1, name2);
    }
    
    pass
}

fn main() {
    let time = 42.0f32;
    
    let mut buffer_libm = [0u8; BUFFER_SIZE];
    let mut buffer_approx = [0u8; BUFFER_SIZE];
    let mut buffer_fixed = [0u8; BUFFER_SIZE];
    let mut buffer_fixed_crate = [0u8; BUFFER_SIZE];
    
    println!("Rendering {}x{} matrix with time={}", WIDTH, HEIGHT, time);
    
    perf_tests_common::perlin3_float_libm::render_frame(&mut buffer_libm, time, WIDTH, HEIGHT);
    perf_tests_common::perlin3_float_approx::render_frame(&mut buffer_approx, time, WIDTH, HEIGHT);
    perf_tests_common::perlin3_fixed::render_frame(&mut buffer_fixed, time, WIDTH, HEIGHT);
    perf_tests_common::perlin3_fixed_crate::render_frame(&mut buffer_fixed_crate, time, WIDTH, HEIGHT);
    
    print_buffer("float_libm (baseline)", &buffer_libm);
    print_buffer("float_approx", &buffer_approx);
    print_buffer("fixed (DIY)", &buffer_fixed);
    print_buffer("fixed_crate", &buffer_fixed_crate);
    
    println!("\n================================================");
    println!("VALIDATION (tolerance: ±10 RGB units):");
    println!("================================================");
    
    let approx_pass = compare_buffers("float_libm", &buffer_libm, "float_approx", &buffer_approx);
    let fixed_pass = compare_buffers("float_libm", &buffer_libm, "fixed (DIY)", &buffer_fixed);
    let crate_pass = compare_buffers("float_libm", &buffer_libm, "fixed_crate", &buffer_fixed_crate);
    let fixed_vs_crate = compare_buffers("fixed (DIY)", &buffer_fixed, "fixed_crate", &buffer_fixed_crate);
    
    println!("\n================================================");
    println!("SUMMARY:");
    println!("================================================");
    println!("float_approx:   {}", if approx_pass { "✓ PASS" } else { "✗ FAIL" });
    println!("fixed (DIY):    {}", if fixed_pass { "✓ PASS" } else { "✗ FAIL" });
    println!("fixed_crate:    {}", if crate_pass { "✓ PASS" } else { "✗ FAIL" });
    println!("================================================");
    
    if !approx_pass || !fixed_pass || !crate_pass {
        std::process::exit(1);
    }
}
