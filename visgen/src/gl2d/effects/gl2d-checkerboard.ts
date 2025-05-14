import { Vec4Data, Vec4Param } from "../../params/vec4-param.ts";
import type { Gl2dContext } from "../gl2d-context.ts";
import { Gl2dFragmentShader } from "../gl2d-fragment-shader.ts";
import { Gl2dEffect } from "./gl2d-effect.ts";

export const Gl2dCheckerboard = Gl2dEffect(
  {
    color1: Vec4Param({ default: [1, 1, 1, 1] }),
    color2: Vec4Param({ default: [0, 0, 0, 1] }),
  },
  (context: Gl2dContext) => {
    const shader = Gl2dFragmentShader(context, glsl);

    return {
      draw({ color1, color2 }: { color1: Vec4Data; color2: Vec4Data }) {
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
);

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
