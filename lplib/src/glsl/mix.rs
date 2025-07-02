use super::Vec2;
use super::Vec3;
use super::Vec4;

/// GLSL-like mix function
/// Linear interpolation between x and y using a to weight between them
pub fn mix<T, W>(x: T, y: T, a: W) -> T 
where
    T: Mix<W>,
{
    T::mix(x, y, a)
}

/// Trait for types that support linear interpolation
pub trait Mix<W> {
    fn mix(x: Self, y: Self, weight: W) -> Self;
}

impl Mix<f32> for f32 {
    fn mix(x: Self, y: Self, weight: f32) -> Self {
        x * (1.0 - weight) + y * weight
    }
}

impl Mix<f32> for Vec2 {
    fn mix(x: Self, y: Self, weight: f32) -> Self {
        Vec2::new(
            mix(x.x, y.x, weight),
            mix(x.y, y.y, weight)
        )
    }
}

impl Mix<Vec2> for Vec2 {
    fn mix(x: Self, y: Self, weight: Vec2) -> Self {
        Vec2::new(
            mix(x.x, y.x, weight.x),
            mix(x.y, y.y, weight.y)
        )
    }
}

impl Mix<f32> for Vec4 {
    fn mix(x: Self, y: Self, weight: f32) -> Self {
        Vec4::new(
            mix(x.x, y.x, weight),
            mix(x.y, y.y, weight),
            mix(x.z, y.z, weight),
            mix(x.w, y.w, weight)
        )
    }
}

impl Mix<f32> for [f32; 4] {
    fn mix(x: Self, y: Self, weight: f32) -> Self {
        [
            mix(x[0], y[0], weight),
            mix(x[1], y[1], weight),
            mix(x[2], y[2], weight),
            mix(x[3], y[3], weight),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_f32() {
        assert_eq!(mix(0.0, 1.0, 0.5), 0.5);
        assert_eq!(mix(0.0, 1.0, 0.0), 0.0);
        assert_eq!(mix(0.0, 1.0, 1.0), 1.0);
    }

    #[test]
    fn test_mix_vec2() {
        let v1 = Vec2::new(0.0, 1.0);
        let v2 = Vec2::new(1.0, 0.0);
        let result = mix(v1, v2, 0.5);
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, 0.5);
    }

    #[test]
    fn test_mix_vec2_vec2() {
        let v1 = Vec2::new(0.0, 1.0);
        let v2 = Vec2::new(1.0, 0.0);
        let a = Vec2::new(0.5, 0.25);
        let result = mix(v1, v2, a);
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, 0.75);
    }

    #[test]
    fn test_mix_vec4() {
        let v1 = Vec4::new(1.0, 0.0, 0.0, 1.0); // Red
        let v2 = Vec4::new(0.0, 0.0, 1.0, 1.0); // Blue
        let result = mix(v1, v2, 0.5);
        assert_eq!(result.x, 0.5); // R
        assert_eq!(result.y, 0.0); // G
        assert_eq!(result.z, 0.5); // B
        assert_eq!(result.w, 1.0); // A
    }

    #[test]
    fn test_mix_color() {
        let c1 = [1.0, 0.0, 0.0, 1.0]; // Red
        let c2 = [0.0, 0.0, 1.0, 1.0]; // Blue
        let result = mix(c1, c2, 0.5);
        assert_eq!(result[0], 0.5); // R
        assert_eq!(result[1], 0.0); // G
        assert_eq!(result[2], 0.5); // B
        assert_eq!(result[3], 1.0); // A
    }
} 