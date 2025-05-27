import { ImageDef } from "../../data/types/image-def.tsx";
import { Vec3Def } from "../../data/types/vec3-def";
import { RecordDef } from "../../data/types/record-def";
import { glsl } from "../../util/glsl";
import { defineGlModule } from "../define-gl-module.tsx";

export const GlGameOfLifeNode = defineGlModule(
  "gl-game-of-life",
  {
    label: "Game of Life",
    params: RecordDef({
      aliveColor: Vec3Def({ default: [1.0, 1.0, 1.0] }),
      deadColor: Vec3Def({ default: [0.0, 0.0, 0.0] }),
    }),
    output: ImageDef(),
  },
  glsl`#version 300 es
    precision highp float;
    
    in vec2 vUv;
    out vec4 fragColor;
    uniform vec2 uResolution;
    uniform sampler2D uInputTexture;
    uniform vec3 uAliveColor;
    uniform vec3 uDeadColor;

    bool isAlive(vec2 uv) {
      if(uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) return false;
      return texture(uInputTexture, uv).r > 0.5;
    }

    void main() {
      vec2 texelSize = 1.0 / uResolution;
      vec2 uv = vUv;
      
      // Count live neighbors
      int neighbors = 0;
      for(int y = -1; y <= 1; y++) {
        for(int x = -1; x <= 1; x++) {
          if(x == 0 && y == 0) continue;
          vec2 offset = vec2(float(x), float(y)) * texelSize;
          if(isAlive(uv + offset)) neighbors++;
        }
      }
      
      // Current cell state
      bool currentlyAlive = isAlive(uv);
      
      // Apply Conway's rules
      bool nextAlive = false;
      if(currentlyAlive) {
        // Any live cell with 2 or 3 live neighbors survives
        nextAlive = (neighbors == 2 || neighbors == 3);
      } else {
        // Any dead cell with exactly 3 live neighbors becomes alive
        nextAlive = (neighbors == 3);
      }
      
      // Output color based on state
      vec3 color = nextAlive ? uAliveColor : uDeadColor;
      fragColor = vec4(color, 1.0);
    }
  `
);
