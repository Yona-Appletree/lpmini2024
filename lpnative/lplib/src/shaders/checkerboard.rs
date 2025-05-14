
use super::super::glsl::Vec4;
use super::super::glsl::Vec2;
use super::super::glsl::mix;
use super::super::glsl::floor;

pub struct CheckerboardInput {
    pub vUv: Vec2,
    pub uResolution: Vec2,
    pub uColor1: Vec4,  // First checkerboard color
    pub uColor2: Vec4,  // Second checkerboard color
}

pub struct CheckerboardOutput {
    pub frag_color: Vec4,
}

pub fn checkerboard(
    CheckerboardInput { 
        vUv, 
        uResolution: _,  // We don't use resolution in this shader
        uColor1, 
        uColor2 
    }: CheckerboardInput
) -> Vec4 {
    let grid = floor(vUv * 8.0);
    let checker = (grid.x + grid.y) % 2.0;
    
    let fragColor = mix(uColor1, uColor2, checker);

    return fragColor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkerboard() {
        let input = CheckerboardInput {
            vUv: Vec2::new(0.5, 0.5),  // Center of the texture
            uResolution: Vec2::new(800.0, 600.0),
            uColor1: Vec4::new(1.0, 0.0, 0.0, 1.0),  // Red
            uColor2: Vec4::new(0.0, 0.0, 1.0, 1.0),  // Blue
        };

        let output = checkerboard(input);
        
        // At the center of a checkerboard pattern, we should get one of the two colors
        assert!(output.x == 1.0 || output.z == 1.0);
    }
} 