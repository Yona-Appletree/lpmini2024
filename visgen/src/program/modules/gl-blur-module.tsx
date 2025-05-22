import { ImageDef } from "../../data/types/image-def.tsx";
import { FloatDef } from "../../data/types/float-def";
import { RecordDef } from "../../data/types/record-def";
import { glsl } from "../../util/glsl";
import { GlModuleDef } from "../gl-module-def.tsx";

export const GlBlurModule = GlModuleDef(
  "gl-blur",
  {
    label: "Blur",
    params: RecordDef({
      radius: FloatDef({
        default: 0.01,
        ui: { type: "slider", min: 0, max: 0.5, step: 0.01 },
      }),
      exponent: FloatDef({ default: 2.0 }),
    }),
    output: ImageDef(),
  },
  glsl`#version 300 es
    precision highp float;
    
    in vec2 vUv;
    out vec4 fragColor;
    uniform vec2 uResolution;
    uniform sampler2D uInputTexture;
    uniform float uRadius;
    uniform float uExponent;

    float gaussian(float x, float sigma) {
      return exp(-pow(x, uExponent) / (2.0 * sigma * sigma)) / (sqrt(2.0 * 3.14159) * sigma);
    }

    void main() {
      vec2 pixelSize = 1.0 / uResolution;
      vec4 color = vec4(0.0);
      
      float sigma = max(1.0, uRadius * 0.5);
      float totalWeight = 0.0;
      
      int kernelSize = int(min(15.0, 2.0 * uRadius + 1.0));
      int halfKernel = kernelSize / 2;
      
      for (int y = -halfKernel; y <= halfKernel; y++) {
        for (int x = -halfKernel; x <= halfKernel; x++) {
          float dist = sqrt(float(x * x + y * y));
          if (dist > float(halfKernel)) continue;
          
          float weight = gaussian(dist, sigma);
          vec2 offset = vec2(float(x), float(y)) * pixelSize;
          color += texture(uInputTexture, vUv + offset) * weight;
          totalWeight += weight;
        }
      }
      
      fragColor = color / totalWeight;
    }
  `,
);
