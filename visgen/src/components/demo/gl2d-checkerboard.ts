import type { Gl2dContext } from "../../gl2d/gl2d-context";
import { Gl2dFragmentShader } from "../../gl2d/gl2d-fragment-shader";

export function Gl2dCheckerboard(context: Gl2dContext) {
  const shader = Gl2dFragmentShader(context, glsl);

  return {
    draw(
      color1: [number, number, number, number],
      color2: [number, number, number, number]
    ) {
      shader.draw({
        uColor1: {
          type: "vec4",
          value: color1,
        },
        uColor2: {
          type: "vec4",
          value: color2,
        },
      });
    },
    [Symbol.dispose]() {
      shader[Symbol.dispose]();
    },
  };
}
export type Gl2dCheckerboard = ReturnType<typeof Gl2dCheckerboard>;

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
