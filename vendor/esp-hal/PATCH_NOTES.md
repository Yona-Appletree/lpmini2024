# esp-hal Vendor Patch

## Source
- Registry version: `esp-hal = "1.0.0-rc.0"` from crates.io
- Git commit (if applicable): `990af189ae7d5dc80ce0bbd0c379952b54f6b1c1`

## Change Made
Removed `"portable-atomic/unsafe-assume-single-core"` from the `esp32c3` feature in `Cargo.toml`.

### Before (registry version):
```toml
esp32c3 = [
    "dep:esp32c3",
    "esp-riscv-rt/rtc-ram",
    "portable-atomic/unsafe-assume-single-core",  # ← This line removed
    "esp-rom-sys/esp32c3",
    "esp-metadata-generated/esp32c3",
]
```

### After (vendor copy):
```toml
esp32c3 = [
    "dep:esp32c3",
    "esp-riscv-rt/rtc-ram",
    "esp-rom-sys/esp32c3",
    "esp-metadata-generated/esp32c3",
]
```

## Reason
The `riscv32imc-unknown-none-elf` target (used by ESP32-C3) supports atomic operations natively. The `unsafe-assume-single-core` feature is incompatible with targets that support atomic CAS operations, causing a compile error:

```
error: `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature) 
is not compatible with target that supports atomic CAS
```

## Usage
This vendor copy is used via `[patch.crates-io]` in the workspace `Cargo.toml`:

```toml
[patch.crates-io]
esp-hal = { path = "vendor/esp-hal" }
```

## Other Changes
- Added `build.rs` (copied from registry version, was missing in vendor copy)

