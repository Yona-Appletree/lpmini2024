/// Load coordinate/builtin variable operations
use crate::lpscript::error::RuntimeError;
use crate::math::{Fixed, FIXED_ONE, FIXED_SHIFT};
use crate::test_engine::LoadSource;

/// Execute Load: push built-in variable value onto stack
#[inline(always)]
pub fn exec_load(
    stack: &mut [i32],
    sp: &mut usize,
    source: LoadSource,
    x_norm: Fixed,
    y_norm: Fixed,
    x_int: Fixed,
    y_int: Fixed,
    time: Fixed,
    width: usize,
    height: usize,
) -> Result<(), RuntimeError> {
    if *sp >= 64 {
        return Err(RuntimeError::StackOverflow { sp: *sp });
    }

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
            // Angle from center (0-1 for 0-2π, 0 = east/right)
            let center_x = Fixed::from_i32(width as i32 / 2).0;
            let center_y = Fixed::from_i32(height as i32 / 2).0;
            let dx = x_int.0 - center_x;
            let dy = y_int.0 - center_y;

            // atan2(dy, dx) normalized to 0..1
            if dx == 0 && dy == 0 {
                Fixed::ZERO // Center has no angle
            } else {
                // Approximate atan2 using octants
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

                // Adjust based on quadrant
                Fixed(if dx >= 0 && dy >= 0 {
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
                })
            }
        }
    };

    stack[*sp] = value.0;
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_load_x_norm() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        exec_load(
            &mut stack,
            &mut sp,
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

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 0.5);
    }

    #[test]
    fn test_load_y_norm() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        exec_load(
            &mut stack,
            &mut sp,
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

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 0.75);
    }

    #[test]
    fn test_load_time() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        exec_load(
            &mut stack,
            &mut sp,
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

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 5.5);
    }

    #[test]
    fn test_load_time_norm() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Time = 2.3 should wrap to 0.3
        exec_load(
            &mut stack,
            &mut sp,
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

        assert_eq!(sp, 1);
        assert!((Fixed(stack[0]).to_f32() - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_load_center_dist() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Load at center (50, 50) of 100x100 image
        exec_load(
            &mut stack,
            &mut sp,
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

        assert_eq!(sp, 1);
        // At center, distance should be 0
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_load_center_angle() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Load at center (50, 50) of 100x100 image
        exec_load(
            &mut stack,
            &mut sp,
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

        assert_eq!(sp, 1);
        // At center, angle should be 0 (undefined, we return 0)
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_load_stack_overflow() {
        let mut stack = [0i32; 64];
        let mut sp = 64; // Stack is full

        let result = exec_load(
            &mut stack,
            &mut sp,
            LoadSource::XNorm,
            0.5f32.to_fixed(),
            0.0f32.to_fixed(),
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::ZERO,
            100,
            100,
        );

        assert!(matches!(result, Err(RuntimeError::StackOverflow { sp: 64 })));
    }
}

