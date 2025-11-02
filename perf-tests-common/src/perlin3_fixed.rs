/// Perlin3 noise using fixed-point arithmetic with lookup table
/// This should be the fastest method on ESP32-C3 (no hardware FPU)
use crate::sin_table::SIN_TABLE_I32 as SIN_TABLE;


// Fixed-point format: 16.16 (16 bits integer, 16 bits fractional)
type Fixed = i32;

const FIXED_SHIFT: i32 = 16;
const FIXED_ONE: Fixed = 1 << FIXED_SHIFT;

#[inline(always)]
fn fixed_from_f32(f: f32) -> Fixed {
    (f * FIXED_ONE as f32) as Fixed
}

#[inline(always)]
fn fixed_mul(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * b as i64) >> FIXED_SHIFT) as Fixed
}

#[inline(always)]
fn fixed_div(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * FIXED_ONE as i64) / b as i64) as Fixed
}

/// Fast sine using lookup table
/// Input x should be in fixed-point format representing radians
#[inline(always)]
fn sin_fixed(x: Fixed) -> Fixed {
    // The table covers 0 to 2π mapped to indices 0-255
    // 2π in fixed point ≈ 6.28318 * 65536 ≈ 411775
    const TWO_PI_FIXED: i64 = 411775;

    // Normalize to 0 to 2π range (with wrapping)
    let normalized = ((x as i64).rem_euclid(TWO_PI_FIXED)) as Fixed;

    // Map 0..TWO_PI to 0..255
    let index = ((normalized as i64 * 256) / TWO_PI_FIXED) as usize & 0xFF;
    SIN_TABLE[index]
}

/// Fast cosine using lookup table
#[inline(always)]
fn cos_fixed(x: Fixed) -> Fixed {
    // cos(x) = sin(x + π/2)
    // π/2 in fixed-point ≈ 1.5708 * 65536 ≈ 102944
    const PI_DIV_2_FIXED: Fixed = 102944;
    sin_fixed(x + PI_DIV_2_FIXED)
}

/// Perlin3 noise using fixed-point
#[inline(always)]
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
    fixed_mul(sum, 37450)
}

#[inline(never)]
pub fn render_frame(buffer: &mut [u8], time: f32, width: usize, height: usize) {
    let width_fixed = fixed_from_f32(width as f32);
    let height_fixed = fixed_from_f32(height as f32);
    let time_fixed = fixed_from_f32(time * 0.001);

    for y in 0..height {
        for x in 0..width {
            let x_fixed = fixed_from_f32(x as f32);
            let y_fixed = fixed_from_f32(y as f32);

            let nx = fixed_div(x_fixed * 4, width_fixed);
            let ny = fixed_div(y_fixed * 4, height_fixed);
            let nz = time_fixed;

            let noise = perlin3_fixed(nx, ny, nz);

            // Convert from fixed-point -1.0 to 1.0 range to 0-255
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
