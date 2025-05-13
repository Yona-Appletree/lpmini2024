import type { FragmentShaderCanvas } from "../../util/fragment-shader-canvas";

// Function to run the checkerboard shader with custom colors
export function runCheckerboardShader(
  canvas: FragmentShaderCanvas,
  color1: [number, number, number, number],
  color2: [number, number, number, number]
) {
  canvas.runShader(glsl, {
    uColor1: {
      type: "vec4",
      value: color1,
    },
    uColor2: {
      type: "vec4",
      value: color2,
    },
  });
}

// Checkerboard shader
const glsl = `
      #version 300 es
      precision highp float;
      
      in vec2 vUv;
      out vec4 fragColor;
      uniform vec2 uResolution;
      uniform vec4 uColor1; // First checkerboard color
      uniform vec4 uColor2; // Second checkerboard color

      void main() {
        vec2 uv = vUv;
        vec2 grid = floor(uv * 8.0); // 8x8 grid
        float checker = mod(grid.x + grid.y, 2.0);
        fragColor = mix(uColor1, uColor2, checker);
      }
    `.trim();
