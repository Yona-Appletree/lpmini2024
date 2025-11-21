use crate::dec32::{Dec32, ToDec32};
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
    x_norm: Dec32,
    y_norm: Dec32,
    x_int: Dec32,
    y_int: Dec32,
    time: Dec32,
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
            time % Dec32::ONE
        }
        LoadSource::CenterDist => {
            // Distance from center (0 at center, 1 at farthest corner)
            let center_x: Dec32 = Dec32::from_i32(width as i32 / 2);
            let center_y: Dec32 = Dec32::from_i32(height as i32 / 2);
            let dx: Dec32 = x_int - center_x;
            let dy: Dec32 = y_int - center_y;

            // Use Manhattan distance normalized by half-diagonal
            let manhattan: Dec32 = dx.abs() + dy.abs();
            let max_manhattan: Dec32 = center_x + center_y;
            if max_manhattan.is_zero() {
                Dec32::ZERO
            } else {
                manhattan / max_manhattan
            }
        }
        LoadSource::CenterAngle => {
            // Angle from center in radians (-π to π, 0 = east/right)
            // Compatible with sin/cos which expect radians
            let center_x: Dec32 = Dec32::from_i32(width as i32 / 2);
            let center_y: Dec32 = Dec32::from_i32(height as i32 / 2);
            let dx: Dec32 = x_int - center_x;
            let dy: Dec32 = y_int - center_y;

            // atan2(dy, dx) in radians
            if dx.is_zero() && dy.is_zero() {
                Dec32::ZERO // Center has no angle
            } else {
                // Approximate atan2 using octants (result in 0..1 range)
                let abs_dx: Dec32 = dx.abs();
                let abs_dy: Dec32 = dy.abs();

                let angle: Dec32 = if abs_dx > abs_dy {
                    // Closer to horizontal
                    let ratio: Dec32 = abs_dy / abs_dx;
                    ratio / 8i32.to_dec32() // Scale to ~0..0.125
                } else if !abs_dy.is_zero() {
                    // Closer to vertical
                    let ratio: Dec32 = abs_dx / abs_dy;
                    Dec32::HALF / 2i32.to_dec32() - ratio / 8i32.to_dec32() // 0.25 - scaled ratio
                } else {
                    Dec32::ZERO
                };

                // Adjust based on quadrant to get normalized angle (0..1)
                let normalized: Dec32 = if dx >= Dec32::ZERO && dy >= Dec32::ZERO {
                    // Q1: 0 to 0.25
                    angle
                } else if dx < Dec32::ZERO && dy >= Dec32::ZERO {
                    // Q2: 0.25 to 0.5
                    Dec32::HALF - angle
                } else if dx < Dec32::ZERO && dy < Dec32::ZERO {
                    // Q3: 0.5 to 0.75
                    Dec32::HALF + angle
                } else {
                    // Q4: 0.75 to 1.0
                    Dec32::ONE - angle
                };

                // Convert normalized (0..1) to radians (0..2π)
                // Then shift to -π..π range to match atan2 convention
                let radians: Dec32 = normalized * Dec32::TAU;
                radians - Dec32::PI // Convert 0..2π to -π..π
            }
        }
    };

    stack.push_dec32(value)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dec32::ToDec32;

    #[test]
    fn test_load_x_norm() {
        let mut stack = ValueStack::new(64);

        exec_load(
            &mut stack,
            LoadSource::XNorm,
            0.5f32.to_dec32(),
            0.0f32.to_dec32(),
            Dec32::ZERO,
            Dec32::ZERO,
            Dec32::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.5);
    }

    #[test]
    fn test_load_y_norm() {
        let mut stack = ValueStack::new(64);

        exec_load(
            &mut stack,
            LoadSource::YNorm,
            0.0f32.to_dec32(),
            0.75f32.to_dec32(),
            Dec32::ZERO,
            Dec32::ZERO,
            Dec32::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.75);
    }

    #[test]
    fn test_load_time() {
        let mut stack = ValueStack::new(64);

        exec_load(
            &mut stack,
            LoadSource::Time,
            Dec32::ZERO,
            Dec32::ZERO,
            Dec32::ZERO,
            Dec32::ZERO,
            5.5f32.to_dec32(),
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 5.5);
    }

    #[test]
    fn test_load_time_norm() {
        let mut stack = ValueStack::new(64);

        // Time = 2.3 should wrap to 0.3
        exec_load(
            &mut stack,
            LoadSource::TimeNorm,
            Dec32::ZERO,
            Dec32::ZERO,
            Dec32::ZERO,
            Dec32::ZERO,
            2.3f32.to_dec32(),
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        assert!((Dec32(stack.raw_slice()[0]).to_f32() - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_load_center_dist() {
        let mut stack = ValueStack::new(64);

        // Load at center (50, 50) of 100x100 image
        exec_load(
            &mut stack,
            LoadSource::CenterDist,
            Dec32::ZERO,
            Dec32::ZERO,
            50.0f32.to_dec32(),
            50.0f32.to_dec32(),
            Dec32::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        // At center, distance should be 0
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_load_center_angle() {
        let mut stack = ValueStack::new(64);

        // Load at center (50, 50) of 100x100 image
        exec_load(
            &mut stack,
            LoadSource::CenterAngle,
            Dec32::ZERO,
            Dec32::ZERO,
            50.0f32.to_dec32(),
            50.0f32.to_dec32(),
            Dec32::ZERO,
            100,
            100,
        )
        .unwrap();

        assert_eq!(stack.sp(), 1);
        // At center, angle should be 0 (undefined, we return 0)
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.0);
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
            0.5f32.to_dec32(),
            0.0f32.to_dec32(),
            Dec32::ZERO,
            Dec32::ZERO,
            Dec32::ZERO,
            100,
            100,
        );

        assert!(matches!(result, Err(LpsVmError::StackOverflow { sp: 2 })));
    }
}
