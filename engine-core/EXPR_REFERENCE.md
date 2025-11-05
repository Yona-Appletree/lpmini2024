# Expression Language Reference

The expression language provides a simple, shader-like syntax for generating LED effects.

## Operators

### Arithmetic

- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo

**Note**: For exponentiation, use the `pow(base, exp)` function.

### Comparisons

Returns 0.0 (false) or 1.0 (true):

- `<` Less than
- `>` Greater than
- `<=` Less or equal
- `>=` Greater or equal
- `==` Equal
- `!=` Not equal

### Logical

Treats non-zero as true, returns 0.0 or 1.0:

- `&&` Logical AND
- `||` Logical OR
- `!` Logical NOT

### Ternary

- `condition ? true_val : false_val`

## Built-in Variables

- `xNorm` or `x` - Normalized X coordinate (0..1)
- `yNorm` or `y` - Normalized Y coordinate (0..1)
- `time` or `t` - Time in seconds (fixed-point)
- `timeNorm` - Normalized time (0..1, wraps)
- `centerAngle` or `angle` - Angle from center (0..1)
- `centerDist` or `dist` - Distance from center (0..1+)

## Math Functions

### Basic Math

- `sin(x)` - Sine
- `cos(x)` - Cosine
- `abs(x)` - Absolute value
- `floor(x)` - Round down
- `ceil(x)` - Round up
- `frac(x)` - Fractional part
- `sqrt(x)` - Square root
- `sign(x)` - Returns -1, 0, or 1
- `pow(base, exp)` - Power (integer exponents)
- `min(a, b)` - Minimum
- `max(a, b)` - Maximum

### Clamping & Steps

- `clamp(value, min, max)` - Clamp value to range
- `saturate(x)` - Clamp to 0..1 (HLSL style)
- `step(edge, x)` - Returns 0 if x < edge, else 1

### Interpolation

- `lerp(a, b, t)` or `mix(a, b, t)` - Linear interpolation
- `smoothstep(edge0, edge1, x)` - Smooth Hermite interpolation

### Noise

- `perlin3(x, y, z, octaves)` - 3D Perlin noise with octaves (1-8)
- `perlin3(x, y, z)` - 3D Perlin noise (defaults to 3 octaves)

## Examples

### Simple Gradient

```
xNorm
```

### Animated Wave

```
sin(time + xNorm * 6.28)
```

### Plasma Effect (Current Demo)

```
cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))
```

### Radial Gradient with Step

```
step(0.5, centerDist)
```

### Smooth Pulse

```
smoothstep(0.3, 0.7, sin(time) * 0.5 + 0.5)
```

### Ternary Selection

```
centerDist < 0.5 ? 1.0 : 0.0
```

### Clamped Brightness

```
clamp(sin(time + centerAngle * 6.28), 0.2, 1.0)
```

### Mix Two Patterns

```
lerp(sin(xNorm * 6.28), cos(yNorm * 6.28), saturate(time * 0.1))
```

## Notes

- All math uses 16.16 fixed-point internally
- Trigonometric functions use lookup tables for speed
- Power function only supports integer exponents
- No vectors yet - working with scalars only
