# Roadmap

## Web proof of concept

**Goal**: Configurable, interactive visualizations running in browser. Define
basic architecture, business logic.

Modules are _static_. Their code and configuration shape is defined at compile
time.

Visualizations are defined
in [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language)
and executed using WebGL.

**Modules**

- Input
    - [ ] Mouse/touch
    - [ ] Microphone
    - [ ] Camera
- Visualizations
    - [ ] Perlin noise
    - [ ] Fluid simulation
    - [ ] Cellular automata
- Transformations
- [ ] Various single input
- [ ] Various 2-input
- Mapping
    - [ ] 2d polygon-based mapping to universe-based outputs
- Outputs
    - [ ] 

## Esp32 static modules

**Goal**: Configurable, interactive visualizations running on an esp32s3,
esp32c6, and
in browser.

Modules are _static_. Their code and configuration shape is defined at compile
time.

Visualizations are defined
in [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language)
and source-transpiled to Rust, using a fixed-point implementation of the glsl
types to approximate how they on a GPU.

### Features

- [ ] Initial TypeScript proof of concept of modules and configs.
    - [ ] WebGL visualization modules
    - [ ] Audio and video input modules
    - [ ] LED Mapping output modules
- [ ] GLSL as Rust
    - [ ] Fixed point API
    - [ ] GLSL Transpiler
- [ ] Rust engine running in `wasm` acting as a device emulator
-

## Dynamic modules on linux

## Dynamic modules on embedded

# Architecture

At runtime, a `LightProgram` is the core unit of execution for the
visualization. It consists of a number of `LightModules` which have
a `_index` that defines how they get their `input` values,
which are either `literal` or an `expression`.

## Modules

