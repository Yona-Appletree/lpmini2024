import { ImageDef } from "../../data/types/image-def.tsx";
import { FloatDef } from "../../data/types/float-def";
import { RecordDef } from "../../data/types/record-def";
import { glsl } from "../../util/glsl";
import { GlModuleDef } from "../gl-module-def.tsx";

export const GlPolarScrollNode = GlModuleDef(
  "gl-polar-scroll",
  {
    label: "Polar Scroll",
    params: RecordDef({
      offset: FloatDef({ default: 0 }),
    }),
    output: ImageDef(),
  },
  glsl`#version 300 es
    precision highp float;
    
    in vec2 vUv;
    out vec4 fragColor;
    uniform vec2 uResolution;
    uniform sampler2D uInputTexture;
    uniform float uOffset;

    void main() {
      // Convert UV to normalized device coordinates (-1 to 1)
      vec2 ndc = vUv * 2.0 - 1.0;
      
      // Convert to polar coordinates
      float r = length(ndc);
      float theta = atan(ndc.y, ndc.x);
      
      // Apply offset to radius and wrap around
      float newR = fract(r + uOffset);
      
      // Convert back to Cartesian coordinates
      vec2 newNdc = vec2(cos(theta), sin(theta)) * newR;
      
      // Convert back to UV space
      vec2 newUv = (newNdc + 1.0) * 0.5;
      
      // Sample with proper filtering
      vec2 pixelSize = 1.0 / uResolution;
      vec4 color = vec4(0.0);
      float totalWeight = 0.0;
      
      // Sample multiple points to avoid artifacts
      for (int i = -1; i <= 1; i++) {
        for (int j = -1; j <= 1; j++) {
          vec2 offset = vec2(float(i), float(j)) * pixelSize;
          float weight = 1.0 - length(vec2(i, j)) / 2.0;
          color += texture(uInputTexture, newUv + offset) * weight;
          totalWeight += weight;
        }
      }
      
      fragColor = color / totalWeight;
    }
  `,
);
