import { FloatParam } from "../../effect-param/params/float-param.ts";
import { Gl2dEffectDef } from "../gl2d-effect-def.ts";
import { glsl } from "../../util/glsl.ts";

export const Gl2dRotate = Gl2dEffectDef(
  "rotate",
  {
    params: {
      angleNorm: FloatParam({ default: 0 }),
      swirlNorm: FloatParam({ default: 2.0 }),
    },
  },
  glsl`
    #version 300 es
    precision highp float;
    
    in vec2 vUv;
    out vec4 fragColor;
    uniform vec2 uResolution;
    uniform sampler2D uInputTexture;
    uniform float uAngle;
    uniform float uSwirlFactor;

    const float PI = 3.14159265359;
    
    void main() {
      // Convert UV to normalized device coordinates (-1 to 1)
      vec2 ndc = vUv * 2.0 - 1.0;
      
      // Calculate distance from center (0 to 1)
      float dist = length(ndc);
      
      // Calculate angle based on distance - more rotation further from center
      // uSwirlFactor of 1.0 means one full rotation at the edges
      float angle = uAngle + (dist * uSwirlFactor * 2.0 * PI);
      
      // Create rotation matrix
      float cosAngle = cos(angle);
      float sinAngle = sin(angle);
      mat2 rotationMatrix = mat2(
        cosAngle, -sinAngle,
        sinAngle, cosAngle
      );
      
      // Apply rotation
      vec2 rotatedNdc = rotationMatrix * ndc;
      
      // Convert back to UV space
      vec2 newUv = (rotatedNdc + 1.0) * 0.5;
      
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
  `
);

export type Gl2dRotate = ReturnType<typeof Gl2dRotate>;
