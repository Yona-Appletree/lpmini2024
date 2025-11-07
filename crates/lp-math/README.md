# lp-math

Fixed-point math library for embedded systems.

## Overview

`lp-math` provides a fixed-point arithmetic library optimized for embedded and `no_std` environments. It uses 16.16 fixed-point representation for efficient math operations without floating-point hardware.

## Features

- **16.16 fixed-point arithmetic** with overflow protection
- **Vector types**: `Vec2`, `Vec3`, `Vec4` with swizzling support
- **Trigonometry**: Sine, cosine, tangent with lookup tables
- **Math utilities**: Square root, power, interpolation, clamping
- **Perlin noise**: 3D noise generation with octave support
- **no_std compatible** with optional `serde` support

## Core Types

- `Fixed` - 16.16 fixed-point number
- `Vec2`, `Vec3`, `Vec4` - Fixed-point vectors

## Usage

### Basic Fixed-Point Math

```rust
use lp_math::fixed::{Fixed, ToFixed};

// Convert from integers and floats
let a = 5i32.to_fixed();
let b = 1.5f32.to_fixed();

// Arithmetic operations
let c = a + b;  // 6.5
let d = a * b;  // 7.5

// Constants
use lp_math::fixed::{ONE, ZERO, HALF};
let half_value = a * HALF;
```

### Vectors

```rust
use lp_math::fixed::{Vec2, Vec3, ToFixed};

// Create vectors
let v1 = Vec2::new(1.0.to_fixed(), 2.0.to_fixed());
let v2 = Vec3::new(0.5.to_fixed(), 0.5.to_fixed(), 1.0.to_fixed());

// Vector operations
let length = v1.length();
let normalized = v1.normalize();
let dot = v1.dot(v2);
```

### Math Functions

```rust
use lp_math::fixed::{sin, cos, sqrt, lerp, clamp, ToFixed};

// Trigonometry
let angle = 1.57.to_fixed();  // π/2
let sine = sin(angle);
let cosine = cos(angle);

// Math utilities
let root = sqrt(4.0.to_fixed());
let interpolated = lerp(0.0.to_fixed(), 10.0.to_fixed(), 0.5.to_fixed());
let clamped = clamp(15.0.to_fixed(), 0.0.to_fixed(), 10.0.to_fixed());
```

### Noise Generation

```rust
use lp_math::fixed::{perlin3, ToFixed};

// 3D Perlin noise
let x = 1.5.to_fixed();
let y = 2.0.to_fixed();
let z = 0.5.to_fixed();
let noise_value = perlin3(x, y, z, 3);  // 3 octaves
```

## Constants

- `ONE` - 1.0 in fixed-point
- `ZERO` - 0.0 in fixed-point
- `HALF` - 0.5 in fixed-point
- `PI` - π in fixed-point
- `TWO_PI` - 2π in fixed-point

## Feature Flags

- `serde`: Enable serialization/deserialization support
