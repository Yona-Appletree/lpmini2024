import type { Gl2dContext } from "../../gl2d/gl2d-context";
import { Gl2dFragmentShader } from "../../gl2d/gl2d-fragment-shader";

export function Gl2dBlur(context: Gl2dContext) {
  const shader = Gl2dFragmentShader(context, glsl);

  return {
    draw(radius: number) {
      shader.draw({
        uBlurRadius: {
          type: "float",
          value: radius,
        },
      });
    },
    [Symbol.dispose]() {
      shader[Symbol.dispose]();
    },
  };
}

export type Gl2dBlur = ReturnType<typeof Gl2dBlur>;

const glsl = `
      #version 300 es
      precision highp float;
      
      in vec2 vUv;
      out vec4 fragColor;
      uniform vec2 uResolution;
      uniform sampler2D uInputTexture;
      uniform float uBlurRadius; // Blur radius in pixels

      // Calculate Gaussian weight based on distance and sigma
      float gaussian(float x, float sigma) {
        return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * 3.14159) * sigma);
      }

      void main() {
        vec2 pixelSize = 1.0 / uResolution;
        vec4 color = vec4(0.0);
        
        // Calculate sigma based on radius
        float sigma = max(1.0, uBlurRadius * 0.5);
        float totalWeight = 0.0;
        
        // Calculate kernel size based on radius (2 * radius + 1)
        int kernelSize = int(min(15.0, 2.0 * uBlurRadius + 1.0));
        int halfKernel = kernelSize / 2;
        
        // Horizontal blur
        for (int i = -halfKernel; i <= halfKernel; i++) {
          float weight = gaussian(float(i), sigma);
          vec2 offset = vec2(float(i)) * pixelSize;
          color += texture(uInputTexture, vUv + offset) * weight;
          totalWeight += weight;
        }
        color /= totalWeight;
        
        // Vertical blur
        vec4 finalColor = vec4(0.0);
        totalWeight = 0.0;
        for (int i = -halfKernel; i <= halfKernel; i++) {
          float weight = gaussian(float(i), sigma);
          vec2 offset = vec2(0.0, float(i)) * pixelSize;
          finalColor += texture(uInputTexture, vUv + offset) * weight;
          totalWeight += weight;
        }
        finalColor /= totalWeight;
        
        fragColor = finalColor;
      }
    `.trim();
