import { FloatParam } from "../../effect-param/params/float-param.ts";
import { Gl2dEffectDef } from "../gl2d-effect-def.ts";
import { glsl } from "../../util/glsl.ts";

export const Gl2dBlur = Gl2dEffectDef(
  "blur",
  {
    params: {
      radius: FloatParam({
        default: 0.01,
        min: 0,
        max: 0.5,
        step: 0.01,
      }),
      exponent: FloatParam({
        default: 2.0,
      }),
    },
  },
  glsl`
    #version 300 es
    precision highp float;
    
    in vec2 vUv;
    out vec4 fragColor;
    uniform vec2 uResolution;
    uniform sampler2D uInputTexture;
    uniform float uRadius; // Blur radius in pixels
    uniform float uExponent; // Controls the shape of the Gaussian kernel

    // Calculate Gaussian weight based on distance and sigma
    float gaussian(float x, float sigma) {
      return exp(-pow(x, uExponent) / (2.0 * sigma * sigma)) / (sqrt(2.0 * 3.14159) * sigma);
    }

    void main() {
      vec2 pixelSize = 1.0 / uResolution;
      vec4 color = vec4(0.0);
      
      // Calculate sigma based on radius
      float sigma = max(1.0, uRadius * 0.5);
      float totalWeight = 0.0;
      
      // Calculate kernel size based on radius
      int kernelSize = int(min(15.0, 2.0 * uRadius + 1.0));
      int halfKernel = kernelSize / 2;
      
      // Circular Gaussian blur
      for (int y = -halfKernel; y <= halfKernel; y++) {
        for (int x = -halfKernel; x <= halfKernel; x++) {
          // Calculate distance from center
          float dist = sqrt(float(x * x + y * y));
          // Skip samples outside the circle
          if (dist > float(halfKernel)) continue;
          
          float weight = gaussian(dist, sigma);
          vec2 offset = vec2(float(x), float(y)) * pixelSize;
          color += texture(uInputTexture, vUv + offset) * weight;
          totalWeight += weight;
        }
      }
      
      fragColor = color / totalWeight;
    }
  `
);

export type Gl2dBlur = ReturnType<typeof Gl2dBlur>;
