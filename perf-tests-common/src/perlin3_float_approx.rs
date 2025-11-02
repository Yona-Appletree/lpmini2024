/// Perlin3 noise using floating-point with fast approximations
/// Uses polynomial approximations for sin/cos instead of libm

/// Fast sine approximation using a polynomial
/// Valid for -PI to PI, uses Bhaskara I's sine approximation
#[inline(always)]
fn sin_approx(mut x: f32) -> f32 {
    const PI: f32 = 3.14159265359;
    const TWO_PI: f32 = 2.0 * PI;
    
    // Normalize to -PI to PI range
    x = x % TWO_PI;
    if x > PI {
        x -= TWO_PI;
    } else if x < -PI {
        x += TWO_PI;
    }
    
    // Bhaskara I's approximation
    // sin(x) ≈ 16x(π - |x|) / (5π² - 4|x|(π - |x|))
    let abs_x = if x < 0.0 { -x } else { x };
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    
    let numerator = 16.0 * abs_x * (PI - abs_x);
    let denominator = 5.0 * PI * PI - 4.0 * abs_x * (PI - abs_x);
    
    sign * numerator / denominator
}

/// Fast cosine approximation
#[inline(always)]
fn cos_approx(x: f32) -> f32 {
    const PI_2: f32 = 3.14159265359 / 2.0;
    sin_approx(x + PI_2)
}

/// Perlin3 noise using approximations
#[inline(always)]
fn perlin3(x: f32, y: f32, z: f32) -> f32 {
    let freq1 = 1.0;
    let freq2 = 2.0;
    let freq3 = 4.0;

    let n1 = sin_approx(x * freq1 + z) * cos_approx(y * freq1);
    let n2 = sin_approx(x * freq2 - z) * cos_approx(y * freq2) * 0.5;
    let n3 = sin_approx(x * freq3 + y + z) * 0.25;

    (n1 + n2 + n3) / 1.75
}

#[inline(never)]
pub fn render_frame(buffer: &mut [u8], time: f32, width: usize, height: usize) {
    let width_f = width as f32;
    let height_f = height as f32;

    for y in 0..height {
        for x in 0..width {
            let nx = (x as f32 / width_f) * 4.0;
            let ny = (y as f32 / height_f) * 4.0;
            let nz = time * 0.001;

            let noise = perlin3(nx, ny, nz);
            let value = ((noise + 1.0) * 0.5 * 255.0) as u8;

            let idx = (y * width + x) * 3;
            buffer[idx] = value;
            buffer[idx + 1] = value;
            buffer[idx + 2] = value;
        }
    }
}

