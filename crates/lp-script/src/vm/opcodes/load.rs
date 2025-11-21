use crate::fixed::{Fixed, ToFixed};
/// Load coordinate/builtin variable operations
use crate::vm::error::LpsVmError;
use crate::vm::value_stack::ValueStack;

/// Load source specifier for built-in variables
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadSource {
    XInt,        // Integer X coordinate (0..width-1)
    YInt,        // Integer Y coordinate (0..height-1)
    XNorm,       // Normalized X (0..1)
    YNorm,       // Normalized Y (0..1)
    Time,        // Time value
    TimeNorm,    // Time normalized to 0..1 range (wraps at 1.0)
    CenterDist,  // Distance from center (0 at center, 1 at farthest corner)
    CenterAngle, // Angle from center (0-1 for 0-2π, 0 = east/right)
}

/// Execute Load: push built-in variable value onto stack
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn exec_load(
    stack: &mut ValueStack,
    source: LoadSource,
    x_norm: Fixed,
    y_norm: Fixed,
    x_int: Fixed,
    y_int: Fixed,
    time: Fixed,
    width: usize,
    height: usize,
) -> Result<(), LpsVmError> {
    let value = match source {
        LoadSource::XNorm => x_norm,
        LoadSource::YNorm => y_norm,
        LoadSource::XInt => x_int,
        LoadSource::YInt => y_int,
        LoadSource::Time => time,
        LoadSource::TimeNorm => {
            // Wrap time to 0..1 range
            time % Fixed::ONE
        }
        LoadSource::CenterDist => {
            // Distance from center (0 at center, 1 at farthest corner)
            let center_x: Fixed = Fixed::from_i32(width as i32 / 2);
            let center_y: Fixed = Fixed::from_i32(height as i32 / 2);
            let dx: Fixed = x_int - center_x;
            let dy: Fixed = y_int - center_y;

            // Use Manhattan distance normalized by half-diagonal
            let manhattan: Fixed = dx.abs() + dy.abs();
            let max_manhattan: Fixed = center_x + center_y;
            if max_manhattan.is_zero() {
                Fixed::ZERO
            } else {
                manhattan / max_manhattan
            }
        }
        LoadSource::CenterAngle => {
            // Angle from center in radians (-π to π, 0 = east/right)
            // Compatible with sin/cos which expect radians
            let center_x: Fixed = Fixed::from_i32(width as i32 / 2);
            let center_y: Fixed = Fixed::from_i32(height as i32 / 2);
            let dx: Fixed = x_int - center_x;
            let dy: Fixed = y_int - center_y;

            // atan2(dy, dx) in radians
            if dx.is_zero() && dy.is_zero() {
                Fixed::ZERO // Center has no angle
            } else {
                // Approximate atan2 using octants (result in 0..1 range)
                let abs_dx: Fixed = dx.abs();
                let abs_dy: Fixed = dy.abs();

                let angle: Fixed = if abs_dx > abs_dy {
                    // Closer to horizontal
                    let ratio: Fixed = abs_dy / abs_dx;
                    ratio / 8i32.to_fixed() // Scale to ~0..0.125
                } else if !abs_dy.is_zero() {
                    // Closer to vertical
                    let ratio: Fixed = abs_dx / abs_dy;
                    Fixed::HALF / 2i32.to_fixed() - ratio / 8i32.to_fixed() // 0.25 - scaled ratio
                } else {
                    Fixed::ZERO
                };

                // Adjust based on quadrant to get normalized angle (0..1)
                let normalized: Fixed = if dx >= Fixed::ZERO && dy >= Fixed::ZERO {
                    // Q1: 0 to 0.25
                    angle
                } else if dx < Fixed::ZERO && dy >= Fixed::ZERO {
                    // Q2: 0.25 to 0.5
                    Fixed::HALF - angle
                } else if dx < Fixed::ZERO && dy < Fixed::ZERO {
                    // Q3: 0.5 to 0.75
                    Fixed::HALF + angle
                } else {
                    // Q4: 0.75 to 1.0
                    Fixed::ONE - angle
                };

                // Convert normalized (0..1) to radians (0..2π)
                // Then shift to -π..π range to match atan2 convention
                let radians: Fixed = normalized * Fixed::TAU;
                radians - Fixed::PI // Convert 0..2π to -π..π
            }
        }
    };

    stack.push_fixed(value)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixed::ToFixed;

    #[test]
    fn test_load_x_norm() {
        let mut stack = ValueStack::new(64);

        exec_load(
            &mut stack,
            LoadSource::XNorm,
            0.5f32.to_fixed(),
            0.0f32.to_fixed(),
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.5);
    }

    #[test]
    fn test_load_y_norm() {
        let mut stack = ValueStack::new(64);

        exec_load(
            &mut stack,
            LoadSource::YNorm,
            0.0f32.to_fixed(),
            0.75f32.to_fixed(),
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.75);
    }

    #[test]
    fn test_load_time() {
        let mut stack = ValueStack::new(64);

        exec_load(
            &mut stack,
            LoadSource::Time,
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            5.5f32.to_fixed(),
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 5.5);
    }

    #[test]
    fn test_load_time_norm() {
        let mut stack = ValueStack::new(64);

        // Time = 2.3 should wrap to 0.3
        exec_load(
            &mut stack,
            LoadSource::TimeNorm,
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            2.3f32.to_fixed(),
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert!((Fixed(stack.raw_slice()[0]).to_f32() - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_load_center_dist() {
        let mut stack = ValueStack::new(64);

        // Load at center (50, 50) of 100x100 image
        exec_load(
            &mut stack,
            LoadSource::CenterDist,
            Fixed::ZERO,
            Fixed::ZERO,
            50.0f32.to_fixed(),
            50.0f32.to_fixed(),
            Fixed::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        // At center, distance should be 0
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_load_center_angle() {
        let mut stack = ValueStack::new(64);

        // Load at center (50, 50) of 100x100 image
        exec_load(
            &mut stack,
            LoadSource::CenterAngle,
            Fixed::ZERO,
            Fixed::ZERO,
            50.0f32.to_fixed(),
            50.0f32.to_fixed(),
            Fixed::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        // At center, angle should be 0 (undefined, we return 0)
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_load_stack_overflow() {
        let mut stack = ValueStack::new(2); // Small stack
                                            // Fill the stack
        stack.push_int32(1).unwrap();
        stack.push_int32(2).unwrap();

        let result = exec_load(
            &mut stack,
            LoadSource::XNorm,
            0.5f32.to_fixed(),
            0.0f32.to_fixed(),
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            100,
            100,
        );

        assert!(matches!(result, Err(LpsVmError::StackOverflow { sp: 2 })));
    }
}
