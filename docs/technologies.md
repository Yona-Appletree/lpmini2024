# Technologies

## Rust

Rust is the primary language for the core engine of lightplayer.

It is chosen because:

- it can **scale** from a single core microcontroller with limited memory
  to a many-core sever with gigabytes of memory.
- emphasizes **correctness** with its type system, ownership model,
  and built-in testing

- Rust on esp32 book: https://docs.esp-rs.org/book/
- Embassy framework: https://github.com/embassy-rs/embassy?tab=readme-ov-file
-

## Svelte

Svelte is used for the web frontend.

## TOML

Toml is used for configuration because it is **simple** and well supported
in the rust ecosystem.

## GLSL

GLSL is used to write the visualizations

- Rust SPIR-V tools: https://github.com/gfx-rs/rspirv
- https://github.com/PENGUINLIONG/inline-spirv-rs
- ## SPIR-V VM https://github.com/daseyb/otherside
  Reddit: https://www.reddit.com/r/vulkan/comments/3doa1e/otherside_a_vm_running_spirv_code_on_the_cpu/
- Newer SPIR-V VM https://github.com/dfranx/SPIRV-VM
- ShaderEd tool https://github.com/dfranx/SHADERed
