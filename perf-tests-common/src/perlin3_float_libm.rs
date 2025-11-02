/// Perlin3 noise using floating-point with libm (baseline)
/// This is the slowest but most accurate method

/// Simple perlin-like noise function
#[inline(always)]
fn perlin3(x: f32, y: f32, z: f32) -> f32 {
    let freq1 = 1.0;
    let freq2 = 2.0;
    let freq3 = 4.0;

    // Use std sin/cos when available, libm otherwise
    #[cfg(feature = "use-libm")]
    {
        let n1 = libm::sinf(x * freq1 + z) * libm::cosf(y * freq1);
        let n2 = libm::sinf(x * freq2 - z) * libm::cosf(y * freq2) * 0.5;
        let n3 = libm::sinf(x * freq3 + y + z) * 0.25;
        (n1 + n2 + n3) / 1.75
    }
    
    #[cfg(not(feature = "use-libm"))]
    {
        let n1 = (x * freq1 + z).sin() * (y * freq1).cos();
        let n2 = (x * freq2 - z).sin() * (y * freq2).cos() * 0.5;
        let n3 = (x * freq3 + y + z).sin() * 0.25;
        (n1 + n2 + n3) / 1.75
    }
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

