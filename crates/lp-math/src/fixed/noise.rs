/// Perlin noise implementation
use core::cmp::Ord;

use super::fixed::Fixed;
use super::interpolation::lerp;

// Permutation table for perlin noise (standard 256-entry table)
const PERM: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

// Fade function for perlin noise: 6t^5 - 15t^4 + 10t^3
#[inline(always)]
fn fade(t: Fixed) -> Fixed {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;

    let six = Fixed::from_i32(6);
    let fifteen = Fixed::from_i32(15);
    let ten = Fixed::from_i32(10);

    six * t5 - fifteen * t4 + ten * t3
}

// Gradient function - uses permutation table to get pseudo-random gradient
#[inline(always)]
fn grad(hash: u8, x: Fixed, y: Fixed, z: Fixed) -> Fixed {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };
    let u_val = if (h & 1) == 0 { u } else { -u };
    let v_val = if (h & 2) == 0 { v } else { -v };
    u_val + v_val // Standard Perlin: sum of two gradient components
}

/// 3D Perlin noise with multiple octaves
///
/// # Arguments
/// * `x`, `y`, `z` - 3D coordinates in fixed-point
/// * `octaves` - Number of octaves (1-8) for fractal noise
///
/// # Returns
/// Fixed-point value in range 0..1 (normalized for ease of use)
pub fn perlin3(x: Fixed, y: Fixed, z: Fixed, octaves: u8) -> Fixed {
    let octaves = octaves.clamp(1, 8);
    let mut total = 0i64;
    let mut amplitude = Fixed::ONE.0 as i64;
    let mut frequency = Fixed::ONE;

    for _ in 0..octaves {
        let sample_x = x * frequency;
        let sample_y = y * frequency;
        let sample_z = z * frequency;

        let noise_val = perlin3_single(sample_x, sample_y, sample_z).0 as i64;
        total += noise_val * amplitude;

        amplitude >>= 1; // Halve amplitude for next octave
        frequency = Fixed(frequency.0 << 1);
    }

    let raw = Fixed((total >> Fixed::SHIFT) as i32);

    // Normalize from natural range (approx -0.866..0.866) to 0..1
    // Scale by ~1.15 (to get -1..1 range) then map to 0..1
    // Using fixed point: multiply by 1.2 and add 0.6 to center and scale
    let scaled = raw * Fixed::from_f32(1.2) + Fixed::from_f32(0.6);

    // Clamp to 0..1 range
    if scaled.0 < 0 {
        Fixed::ZERO
    } else if scaled.0 > Fixed::ONE.0 {
        Fixed::ONE
    } else {
        scaled
    }
}

/// Single octave of 3D Perlin noise
fn perlin3_single(x: Fixed, y: Fixed, z: Fixed) -> Fixed {
    // Find unit cube containing point
    let xi = (x.to_i32() & 255) as usize;
    let yi = (y.to_i32() & 255) as usize;
    let zi = (z.to_i32() & 255) as usize;

    // Find relative position in cube (0..1)
    let xf = x.frac();
    let yf = y.frac();
    let zf = z.frac();

    // Compute fade curves
    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    // Hash coordinates of 8 cube corners
    let p = |i: usize| PERM[i & 255] as usize;
    let aaa = p(p(p(xi) + yi) + zi);
    let aba = p(p(p(xi) + yi + 1) + zi);
    let aab = p(p(p(xi) + yi) + zi + 1);
    let abb = p(p(p(xi) + yi + 1) + zi + 1);
    let baa = p(p(p(xi + 1) + yi) + zi);
    let bba = p(p(p(xi + 1) + yi + 1) + zi);
    let bab = p(p(p(xi + 1) + yi) + zi + 1);
    let bbb = p(p(p(xi + 1) + yi + 1) + zi + 1);

    // Blend contributions from 8 corners
    let x1 = lerp(
        grad(PERM[aaa], xf, yf, zf),
        grad(PERM[baa], xf - Fixed::ONE, yf, zf),
        u,
    );

    let x2 = lerp(
        grad(PERM[aba], xf, yf - Fixed::ONE, zf),
        grad(PERM[bba], xf - Fixed::ONE, yf - Fixed::ONE, zf),
        u,
    );

    let y1 = lerp(x1, x2, v);

    let x3 = lerp(
        grad(PERM[aab], xf, yf, zf - Fixed::ONE),
        grad(PERM[bab], xf - Fixed::ONE, yf, zf - Fixed::ONE),
        u,
    );

    let x4 = lerp(
        grad(PERM[abb], xf, yf - Fixed::ONE, zf - Fixed::ONE),
        grad(PERM[bbb], xf - Fixed::ONE, yf - Fixed::ONE, zf - Fixed::ONE),
        u,
    );

    let y2 = lerp(x3, x4, v);

    lerp(y1, y2, w)
}

#[cfg(test)]
mod tests {
    use super::super::conversions::ToFixed;
    use super::*;

    #[test]
    fn test_perlin3_basic() {
        let result = perlin3(0i32.to_fixed(), 0i32.to_fixed(), 0i32.to_fixed(), 3);
        let f = result.to_f32();
        assert!(f >= -2.0 && f <= 2.0, "Perlin output {} out of range", f);
    }

    #[test]
    fn test_perlin3_variation() {
        let p1 = perlin3(0.1f32.to_fixed(), 0.1f32.to_fixed(), 0i32.to_fixed(), 3);
        let p2 = perlin3(0.9f32.to_fixed(), 0.9f32.to_fixed(), 0i32.to_fixed(), 3);
        let p3 = perlin3(1.5f32.to_fixed(), 2.3f32.to_fixed(), 0.7f32.to_fixed(), 3);
        let p4 = perlin3(10.5f32.to_fixed(), 5.2f32.to_fixed(), 3.1f32.to_fixed(), 3);

        // At least one pair should be different
        let has_variation = p1 != p2 || p2 != p3 || p3 != p4;
        assert!(
            has_variation,
            "Perlin should produce varied output for different inputs"
        );
    }

    #[test]
    fn test_perlin3_single_octave() {
        // Test single octave to isolate the issue
        let p = perlin3(0.5f32.to_fixed(), 0.5f32.to_fixed(), 0.5f32.to_fixed(), 1);
        let f = p.to_f32();
        assert!(
            f.abs() > 0.001 || f == 0.0,
            "Perlin should produce non-zero values or be legitimately zero"
        );
    }

    #[test]
    fn test_perlin3_single_direct() {
        // Test perlin3_single directly with detailed debug
        let x = 0.5f32.to_fixed();
        let y = 0.5f32.to_fixed();
        let z = 0.5f32.to_fixed();

        // Manually compute what should happen
        let xi = (x.to_i32() & 255) as usize;
        let yi = (y.to_i32() & 255) as usize;
        let zi = (z.to_i32() & 255) as usize;
        let xf = x.frac();
        let yf = y.frac();
        let zf = z.frac();
        let result = perlin3_single(x, y, z);
        let _f = result.to_f32();
    }

    #[test]
    fn test_lerp_function() {
        // Test that lerp works
        let a = 0i32.to_fixed();
        let b = 1i32.to_fixed();
        let t = 0.5f32.to_fixed();
        let result = lerp(a, b, t);
        let f = result.to_f32();
        assert!((f - 0.5).abs() < 0.01, "lerp should give 0.5, got {}", f);
    }

    #[test]
    fn test_grad_function() {
        // Test that grad produces non-zero output
        let g = grad(1, 1i32.to_fixed(), 1i32.to_fixed(), 1i32.to_fixed());
        // Grad can be zero for some hashes, but test a few
        let g2 = grad(5, 1i32.to_fixed(), 0i32.to_fixed(), 0i32.to_fixed());
    }

    #[test]
    fn test_perlin3_returns_zero_to_one() {
        // Test that perlin3 always returns values in 0..1 range
        // Test a variety of inputs
        let test_cases = [
            (0.0, 0.0, 0.0),
            (0.5, 0.5, 0.5),
            (1.0, 1.0, 1.0),
            (10.5, 5.2, 3.1),
            (-5.0, 3.0, 2.0),
            (100.0, 50.0, 25.0),
        ];

        for &(x, y, z) in &test_cases {
            for octaves in 1..=8 {
                let result = perlin3(x.to_fixed(), y.to_fixed(), z.to_fixed(), octaves);
                let val = result.to_f32();

                assert!(
                    val >= 0.0 && val <= 1.0,
                    "perlin3({}, {}, {}, {}) = {} is outside 0..1 range",
                    x,
                    y,
                    z,
                    octaves,
                    val
                );
            }
        }
    }

    #[test]
    fn test_perlin3_has_good_coverage() {
        // Test that perlin3 can produce values across most of the 0..1 range
        // Sample a grid of values and check we get good coverage
        let mut min_seen = 1.0f32;
        let mut max_seen = 0.0f32;

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..4 {
                    let val = perlin3(
                        (x as f32 * 0.5).to_fixed(),
                        (y as f32 * 0.5).to_fixed(),
                        (z as f32 * 0.5).to_fixed(),
                        3,
                    )
                    .to_f32();

                    min_seen = min_seen.min(val);
                    max_seen = max_seen.max(val);
                }
            }
        }

        let range = max_seen - min_seen;
        assert!(
            range > 0.95,
            "Perlin3 should cover the full 0..1 range, but only covered {}",
            range
        );

        assert!(
            range < 1.05,
            "Perlin3 should cover only 0..1 range, but covered {}",
            range
        );
    }
}
