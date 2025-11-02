/// Debug version of DIY fixed to see what's going wrong

type Fixed = i32;

const FIXED_SHIFT: i32 = 16;
const FIXED_ONE: Fixed = 1 << FIXED_SHIFT;

const SIN_TABLE_SIZE: usize = 256;
const SIN_TABLE: [Fixed; SIN_TABLE_SIZE] = [
         0,       1608,       3215,       4821,       6423,       8022,       9616,      11204,
     12785,      14359,      15923,      17479,      19024,      20557,      22078,      23586,
     25079,      26557,      28020,      29465,      30893,      32302,      33692,      35061,
     36409,      37736,      39039,      40319,      41575,      42806,      44011,      45189,
     46340,      47464,      48558,      49624,      50660,      51665,      52639,      53581,
     54491,      55368,      56212,      57022,      57797,      58538,      59243,      59913,
     60547,      61144,      61705,      62228,      62714,      63162,      63571,      63943,
     64276,      64571,      64826,      65043,      65220,      65358,      65457,      65516,
     65536,      65516,      65457,      65358,      65220,      65043,      64826,      64571,
     64276,      63943,      63571,      63162,      62714,      62228,      61705,      61144,
     60547,      59913,      59243,      58538,      57797,      57022,      56212,      55368,
     54491,      53581,      52639,      51665,      50660,      49624,      48558,      47464,
     46340,      45189,      44011,      42806,      41575,      40319,      39039,      37736,
     36409,      35061,      33692,      32302,      30893,      29465,      28020,      26557,
     25079,      23586,      22078,      20557,      19024,      17479,      15923,      14359,
     12785,      11204,       9616,       8022,       6423,       4821,       3215,       1608,
         0,      -1608,      -3215,      -4821,      -6423,      -8022,      -9616,     -11204,
    -12785,     -14359,     -15923,     -17479,     -19024,     -20557,     -22078,     -23586,
    -25079,     -26557,     -28020,     -29465,     -30893,     -32302,     -33692,     -35061,
    -36409,     -37736,     -39039,     -40319,     -41575,     -42806,     -44011,     -45189,
    -46340,     -47464,     -48558,     -49624,     -50660,     -51665,     -52639,     -53581,
    -54491,     -55368,     -56212,     -57022,     -57797,     -58538,     -59243,     -59913,
    -60547,     -61144,     -61705,     -62228,     -62714,     -63162,     -63571,     -63943,
    -64276,     -64571,     -64826,     -65043,     -65220,     -65358,     -65457,     -65516,
    -65536,     -65516,     -65457,     -65358,     -65220,     -65043,     -64826,     -64571,
    -64276,     -63943,     -63571,     -63162,     -62714,     -62228,     -61705,     -61144,
    -60547,     -59913,     -59243,     -58538,     -57797,     -57022,     -56212,     -55368,
    -54491,     -53581,     -52639,     -51665,     -50660,     -49624,     -48558,     -47464,
    -46340,     -45189,     -44011,     -42806,     -41575,     -40319,     -39039,     -37736,
    -36409,     -35061,     -33692,     -32302,     -30893,     -29465,     -28020,     -26557,
    -25079,     -23586,     -22078,     -20557,     -19024,     -17479,     -15923,     -14359,
    -12785,     -11204,      -9616,      -8022,      -6423,      -4821,      -3215,      -1608,
];

fn fixed_from_f32(f: f32) -> Fixed {
    (f * FIXED_ONE as f32) as Fixed
}

fn fixed_mul(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * b as i64) >> FIXED_SHIFT) as Fixed
}

fn fixed_div(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * FIXED_ONE as i64) / b as i64) as Fixed
}

fn sin_fixed(x: Fixed) -> Fixed {
    const TWO_PI_FIXED: i64 = 411775;
    let normalized = ((x as i64).rem_euclid(TWO_PI_FIXED)) as Fixed;
    let index = ((normalized as i64 * 256) / TWO_PI_FIXED) as usize & 0xFF;
    
    if x == fixed_from_f32(0.0) { // First call only
        println!("  sin_fixed({}) -> normalized={}, index={}, table={}", 
            x as f32 / FIXED_ONE as f32, 
            normalized as f32 / FIXED_ONE as f32,
            index, 
            SIN_TABLE[index] as f32 / FIXED_ONE as f32);
    }
    
    SIN_TABLE[index]
}

fn cos_fixed(x: Fixed) -> Fixed {
    const PI_DIV_2_FIXED: Fixed = 102944;
    sin_fixed(x + PI_DIV_2_FIXED)
}

fn perlin3_fixed(x: Fixed, y: Fixed, z: Fixed) -> Fixed {
    let freq1 = FIXED_ONE;
    let freq2 = FIXED_ONE * 2;
    let freq3 = FIXED_ONE * 4;

    let n1 = fixed_mul(
        sin_fixed(fixed_mul(x, freq1) + z),
        cos_fixed(fixed_mul(y, freq1)),
    );
    let n2 = fixed_mul(
        sin_fixed(fixed_mul(x, freq2) - z),
        cos_fixed(fixed_mul(y, freq2)),
    ) / 2;
    let n3 = sin_fixed(fixed_mul(x, freq3) + y + z) / 4;

    let sum = n1 + n2 + n3;
    
    // Divide by 1.75 = multiply by 1/1.75 ≈ 0.5714
    // In fixed point: 0.5714 * 65536 ≈ 37450
    let result = fixed_mul(sum, 37450);
    
    if x == 0 && y == 0 && false {
        println!("  perlin3: n1={:.4}, n2={:.4}, n3={:.4}, sum={:.4}, result={:.4}",
            n1 as f32 / FIXED_ONE as f32,
            n2 as f32 / FIXED_ONE as f32,
            n3 as f32 / FIXED_ONE as f32,
            sum as f32 / FIXED_ONE as f32,
            result as f32 / FIXED_ONE as f32);
    }
    
    result
}

pub fn render_frame(buffer: &mut [u8], time: f32, width: usize, height: usize) {
    let width_fixed = fixed_from_f32(width as f32);
    let height_fixed = fixed_from_f32(height as f32);
    let time_fixed = fixed_from_f32(time * 0.001);

    // Debug first few pixels
    println!("\n=== DETAILED DEBUG ===");
    println!("FIXED_ONE = {}, width_fixed = {}, height_fixed = {}, time_fixed = {}", 
        FIXED_ONE, width_fixed, height_fixed, time_fixed);
    println!("time={}, time*0.001={}", time, time * 0.001);
    
    // Test the multiply function
    println!("\nTesting fixed_mul:");
    let test_a = FIXED_ONE; // 1.0
    let test_b = FIXED_ONE * 4; // 4.0
    let result = fixed_mul(test_a, test_b);
    println!("  1.0 * 4.0 = {} (expected {})", result as f32 / FIXED_ONE as f32, 4.0);
    
    // The BUG: FIXED_ONE * 4 is treating 4 as an integer multiplier, not fixed-point!
    // Should be: x_fixed * 4 (integer mul), not fixed_mul(x_fixed, FIXED_ONE*4)
    println!("\nTesting coordinate scaling:");
    let x1 = FIXED_ONE; // x=1
    let wrong = fixed_mul(x1, FIXED_ONE * 4);
    let right = x1 * 4; // Integer multiply by 4
    println!("  WRONG: fixed_mul(1.0, FIXED_ONE*4) = {:.6}", wrong as f32 / FIXED_ONE as f32);
    println!("  RIGHT: 1.0 * 4 = {:.6}", right as f32 / FIXED_ONE as f32);
    
    for test_y in 0..2 {
        for test_x in 0..3 {
            let x_f32 = test_x as f32;
            let y_f32 = test_y as f32;
            
            println!("\nPixel ({}, {}):", test_x, test_y);
            
            let x_fixed = fixed_from_f32(x_f32);
            let y_fixed = fixed_from_f32(y_f32);
            println!("  x_fixed={} ({}), y_fixed={} ({})", 
                x_fixed, x_f32,
                y_fixed, y_f32);
            
            let nx_step1 = x_fixed * 4; // Simple integer multiply
            println!("  x * 4 = {} (raw: {})", nx_step1 as f32 / FIXED_ONE as f32, nx_step1);
            
            let nx = fixed_div(nx_step1, width_fixed);
            let ny = fixed_div(y_fixed * 4, height_fixed);
            let nz = time_fixed;
            
            println!("  nx={:.6}, ny={:.6}, nz={:.6}", 
                nx as f32 / FIXED_ONE as f32,
                ny as f32 / FIXED_ONE as f32,
                nz as f32 / FIXED_ONE as f32);
            
            // Compare to float version
            let nx_float = (x_f32 / width as f32) * 4.0;
            let ny_float = (y_f32 / height as f32) * 4.0;
            let nz_float = time * 0.001;
            println!("  Expected (float): nx={:.6}, ny={:.6}, nz={:.6}", 
                nx_float, ny_float, nz_float);
            
            let noise = perlin3_fixed(nx, ny, nz);
            println!("  noise={:.6}", noise as f32 / FIXED_ONE as f32);
            
            let shifted = noise + FIXED_ONE;
            let scaled = (shifted * 255) / (2 * FIXED_ONE);
            let value = scaled.clamp(0, 255) as u8;
            println!("  shifted={}, scaled={}, value={}", shifted, scaled, value);
        }
    }
    println!("=== END DEBUG ===\n");

    for y in 0..height {
        for x in 0..width {
            let x_fixed = fixed_from_f32(x as f32);
            let y_fixed = fixed_from_f32(y as f32);
            
            let nx = fixed_div(x_fixed * 4, width_fixed);
            let ny = fixed_div(y_fixed * 4, height_fixed);
            let nz = time_fixed;

            let noise = perlin3_fixed(nx, ny, nz);
            
            let shifted = noise + FIXED_ONE;
            let scaled = (shifted * 255) / (2 * FIXED_ONE);
            let value = scaled.clamp(0, 255) as u8;

            let idx = (y * width + x) * 3;
            buffer[idx] = value;
            buffer[idx + 1] = value;
            buffer[idx + 2] = value;
        }
    }
}

