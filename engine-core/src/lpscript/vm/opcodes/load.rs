/// Load coordinate/builtin variable operations
use crate::lpscript::vm::error::LpsVmError;
use crate::lpscript::vm::value_stack::ValueStack;
use crate::math::{Fixed, FIXED_ONE, FIXED_SHIFT};

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
            Fixed((time.0 as i64).rem_euclid(FIXED_ONE as i64) as i32)
        }
        LoadSource::CenterDist => {
            // Distance from center (0 at center, 1 at farthest corner)
            let center_x = Fixed::from_i32(width as i32 / 2).0;
            let center_y = Fixed::from_i32(height as i32 / 2).0;
            let dx = x_int.0 - center_x;
            let dy = y_int.0 - center_y;

            // Use Manhattan distance normalized by half-diagonal
            let manhattan = (if dx < 0 { -dx } else { dx }) + (if dy < 0 { -dy } else { dy });
            let max_manhattan = center_x + center_y;
            if max_manhattan == 0 {
                Fixed::ZERO
            } else {
                Fixed(((manhattan as i64 * FIXED_ONE as i64) / max_manhattan as i64) as i32)
            }
        }
        LoadSource::CenterAngle => {
            // Angle from center in radians (-π to π, 0 = east/right)
            // Compatible with sin/cos which expect radians
            let center_x = Fixed::from_i32(width as i32 / 2).0;
            let center_y = Fixed::from_i32(height as i32 / 2).0;
            let dx = x_int.0 - center_x;
            let dy = y_int.0 - center_y;

            // atan2(dy, dx) in radians
            if dx == 0 && dy == 0 {
                Fixed::ZERO // Center has no angle
            } else {
                // Approximate atan2 using octants (result in 0..1 range)
                let abs_dx = if dx < 0 { -dx } else { dx };
                let abs_dy = if dy < 0 { -dy } else { dy };

                let angle = if abs_dx > abs_dy {
                    // Closer to horizontal
                    let ratio = ((abs_dy as i64) << FIXED_SHIFT) / (abs_dx as i64);
                    (ratio as i32) >> 3 // Scale to ~0..0.125
                } else if abs_dy > 0 {
                    // Closer to vertical
                    let ratio = ((abs_dx as i64) << FIXED_SHIFT) / (abs_dy as i64);
                    (FIXED_ONE >> 2) - ((ratio as i32) >> 3) // 0.25 - scaled ratio
                } else {
                    0
                };

                // Adjust based on quadrant to get normalized angle (0..1)
                let normalized = if dx >= 0 && dy >= 0 {
                    // Q1: 0 to 0.25
                    angle
                } else if dx < 0 && dy >= 0 {
                    // Q2: 0.25 to 0.5
                    (FIXED_ONE >> 1) - angle
                } else if dx < 0 && dy < 0 {
                    // Q3: 0.5 to 0.75
                    (FIXED_ONE >> 1) + angle
                } else {
                    // Q4: 0.75 to 1.0
                    FIXED_ONE - angle
                };

                // Convert normalized (0..1) to radians (0..2π)
                // Then shift to -π..π range to match atan2 convention
                let radians =
                    Fixed((normalized as i64 * Fixed::TAU.0 as i64 >> FIXED_SHIFT) as i32);
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
    use crate::math::ToFixed;

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
