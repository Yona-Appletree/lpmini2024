import { describe, expect, it } from "vitest";
import { transpileGlslToRust } from "./transpile-glsl-to-rust";

describe("transpileGlslToRust", () => {
  it("should transpile a basic checkerboard shader", () => {
    const glsl = `
      #version 300 es
      precision highp float;
      
      in vec2 vUv;
      out vec4 fragColor;
      uniform vec2 uResolution;
      uniform vec4 uColor1;
      uniform vec4 uColor2;

      void main() {
        vec2 grid = floor(vUv * 8.0);
        float checker = mod(grid.x + grid.y, 2.0);
        fragColor = mix(uColor1, uColor2, checker);
      }
    `;

    const expectedRust = expect.stringContaining(`
pub struct ShaderInput {
    pub vUv: Vec2,
    pub uResolution: Vec2,
    pub uColor1: Vec4,
    pub uColor2: Vec4,
}

pub struct ShaderOutput {
    pub frag_color: Vec4,
}

pub fn main(
    ShaderInput { vUv, uResolution, uColor1, uColor2 }: ShaderInput
) -> ShaderOutput {
    let grid = floor(vUv * 8.0);
    let checker = mod(grid.x + grid.y, 2.0);
    let frag_color = mix(uColor1, uColor2, checker);
    return frag_color;
}`);

    const result = transpileGlslToRust(glsl);
    expect(result).toMatch(expectedRust);
  });

  it("should handle shaders with no uniforms", () => {
    const glsl = `
      #version 300 es
      precision highp float;
      
      in vec2 vUv;
      out vec4 fragColor;

      void main() {
        fragColor = vec4(vUv.x, vUv.y, 0.0, 1.0);
      }
    `;

    const expectedRust = expect.stringContaining(`
pub struct ShaderInput {
    pub vUv: Vec2,
}

pub struct ShaderOutput {
    pub frag_color: Vec4,
}

pub fn main(
    ShaderInput { vUv }: ShaderInput
) -> ShaderOutput {
    let frag_color = Vec4::new(vUv.x, vUv.y, 0.0, 1.0);
    return frag_color;
}`);

    const result = transpileGlslToRust(glsl);
    expect(result).toMatch(expectedRust);
  });

  it("should handle shaders with multiple outputs", () => {
    const glsl = `
      #version 300 es
      precision highp float;
      
      in vec2 vUv;
      out vec4 fragColor;
      out vec2 extraOutput;

      void main() {
        fragColor = vec4(1.0);
        extraOutput = vUv;
      }
    `;

    const expectedRust = expect.stringContaining(`
pub struct ShaderInput {
    pub vUv: Vec2,
}

pub struct ShaderOutput {
    pub frag_color: Vec4,
    pub extraOutput: Vec2,
}

pub fn main(
    ShaderInput { vUv }: ShaderInput
) -> ShaderOutput {
    let frag_color = Vec4::new(1.0, 1.0, 1.0, 1.0);
    let extraOutput = vUv;
    return ShaderOutput { frag_color, extraOutput };
}`);

    const result = transpileGlslToRust(glsl);
    expect(result).toMatch(expectedRust);
  });

  it("should throw error for invalid GLSL", () => {
    const invalidGlsl = `
      #version 300 es
      precision highp float;
      
      invalid syntax here
    `;

    expect(() => transpileGlslToRust(invalidGlsl)).toThrow();
  });
});
