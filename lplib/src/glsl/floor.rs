use super::Vec2;
use super::Vec3;
use super::Vec4;

/// GLSL-like floor function
/// Returns the largest integer less than or equal to x
pub fn floor<T>(x: T) -> T 
where
    T: Floor,
{
    T::floor(x)
}

/// Trait for types that support floor operation
pub trait Floor {
    fn floor(x: Self) -> Self;
}

impl Floor for f32 {
    fn floor(x: Self) -> Self {
        x.floor()
    }
}

impl Floor for Vec2 {
    fn floor(x: Self) -> Self {
        Vec2::new(
            floor(x.x),
            floor(x.y)
        )
    }
}

impl Floor for Vec3 {
    fn floor(x: Self) -> Self {
        Vec3::new(
            floor(x.x),
            floor(x.y),
            floor(x.z)
        )
    }
}

impl Floor for Vec4 {
    fn floor(x: Self) -> Self {
        Vec4::new(
            floor(x.x),
            floor(x.y),
            floor(x.z),
            floor(x.w)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_f32() {
        assert_eq!(floor(3.7), 3.0);
        assert_eq!(floor(-3.7), -4.0);
        assert_eq!(floor(3.0), 3.0);
    }

    #[test]
    fn test_floor_vec2() {
        let v = Vec2::new(3.7, -3.7);
        let result = floor(v);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, -4.0);
    }

    #[test]
    fn test_floor_vec3() {
        let v = Vec3::new(3.7, -3.7, 3.0);
        let result = floor(v);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_floor_vec4() {
        let v = Vec4::new(3.7, -3.7, 3.0, -3.0);
        let result = floor(v);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, 3.0);
        assert_eq!(result.w, -3.0);
    }
} 