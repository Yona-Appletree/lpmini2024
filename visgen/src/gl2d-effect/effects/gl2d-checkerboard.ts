import { Vec4Param } from "../../effect-param/params/vec4-param.ts";
import { Gl2dEffectDef } from "../gl2d-effect-def.ts";
import { glsl } from "../../util/glsl.ts";
import { IntParam } from "../../effect-param/params/int-param.ts";

export const Gl2dCheckerboard = Gl2dEffectDef(
  "checkerboard",
  {
    params: {
      color1: Vec4Param({ default: [1, 1, 1, 1] }),
      color2: Vec4Param({ default: [0, 0, 0, 1] }),
      rows: IntParam({ default: 8 }),
      columns: IntParam({ default: 8 }),
    },
  },
  glsl`
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
  `,
);

export type Gl2dCheckerboard = ReturnType<typeof Gl2dCheckerboard>;
