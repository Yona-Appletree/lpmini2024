import { TextureDef } from "@/core/data/types/texture-def.tsx";
import { FloatDef } from "@/core/data/types/float-def.tsx";
import { RecordDef } from "@/core/data/types/record-def.tsx";
import { glsl } from "@/frontend/util/glsl.ts";
import { defineGlModule } from "../define-gl-module.tsx";

export const GlBlurModule = defineGlModule(
  "gl-blur",
  {
    label: "Blur",
    params: RecordDef({
      inputTexture: TextureDef(),
      radius: FloatDef({
        default: 0.01,
        ui: { type: "slider", min: 0, max: 0.5, step: 0.01 },
      }),
      exponent: FloatDef({ default: 2.0 }),
    }),
    output: TextureDef(),
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
