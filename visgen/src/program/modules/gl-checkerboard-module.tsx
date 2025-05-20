import { ImageDef } from "../../data/types/image-def.tsx";
import { IntDef } from "../../data/types/int-def.tsx";
import { RecordDef } from "../../data/types/record-def";
import { Vec4Def } from "../../data/types/vec4-def.tsx";
import { glsl } from "../../util/glsl";
import { GlModuleDef } from "../gl-module-def.tsx";

export const GlCheckerboardModule = GlModuleDef(
  "gl-checkerboard",
  {
    label: "Checkerboard",
    params: RecordDef({
      color1: Vec4Def({ default: [1, 1, 1, 1] }),
      color2: Vec4Def({ default: [0, 0, 0, 1] }),
      rows: IntDef({ default: 8 }),
      columns: IntDef({ default: 8 }),
    }),
    output: ImageDef(),
  },
  glsl`
    #version 300 es
    precision highp float;

    in vec2 vUv;
    out vec4 fragColor;
    uniform vec2 uResolution;
    uniform vec4 uColor1; // First checkerboard color
    uniform vec4 uColor2; // Second checkerboard color
    uniform int uRows;    // Number of rows
    uniform int uColumns; // Number of columns

    void main() {
        vec2 uv = vUv;
        vec2 grid = floor(uv * vec2(float(uColumns), float(uRows)));
        float checker = mod(grid.x + grid.y, 2.0);
        fragColor = mix(uColor1, uColor2, checker);
    }
  `,
);
