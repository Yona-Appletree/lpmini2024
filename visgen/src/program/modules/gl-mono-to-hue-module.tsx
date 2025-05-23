import { ImageDef } from "../../data/types/image-def.tsx";
import { FloatDef } from "../../data/types/float-def";
import { RecordDef } from "../../data/types/record-def";
import { glsl } from "../../util/glsl";
import { defineGlModule } from "../define-gl-module.tsx";

export const GlMonoToHueModule = defineGlModule(
  "gl-mono-to-hue",
  {
    label: "Mono to Hue",
    params: RecordDef({
      saturation: FloatDef({ default: 0.8 }),
      luminance: FloatDef({ default: 0.5 }),
      hueShift: FloatDef({ default: 0.0 }),
    }),
    output: ImageDef(),
  },
  glsl`#version 300 es
    precision highp float;

    in vec2 vUv;
    out vec4 fragColor;
    uniform sampler2D uInputTexture;
    uniform float uSaturation;
    uniform float uLuminance;
    uniform float uHueShift;

    float hue2rgb(float p, float q, float t) {
      if (t < 0.0) t += 1.0;
      if (t > 1.0) t -= 1.0;
      if (t < 1.0/6.0) return p + (q - p) * 6.0 * t;
      if (t < 1.0/2.0) return q;
      if (t < 2.0/3.0) return p + (q - p) * (2.0/3.0 - t) * 6.0;
      return p;
    }

    vec3 hsl2rgb(vec3 hsl) {
      float h = hsl.x;
      float s = hsl.y;
      float l = hsl.z;
      float r, g, b;
      if (s == 0.0) {
        r = g = b = l;
      } else {
        float q = l < 0.5 ? l * (1.0 + s) : l + s - l * s;
        float p = 2.0 * l - q;
        r = hue2rgb(p, q, h + 1.0/3.0);
        g = hue2rgb(p, q, h);
        b = hue2rgb(p, q, h - 1.0/3.0);
      }
      return vec3(r, g, b);
    }

    void main() {
      vec4 color = texture(uInputTexture, vUv);
      // Convert to monochrome by taking the average
      float mono = (color.r + color.g + color.b) / 3.0;
      
      // Apply sine function to smooth the result and normalize to [0,1]
      float smoothedHue = (sin(mono * 6.28318530718) + 1.0) * 0.5;
      
      // Add the hue shift and wrap around to [0,1]
      smoothedHue = fract(smoothedHue + uHueShift);
      
      // Create HSL color using the smoothed monochrome value as hue
      vec3 hsl = vec3(smoothedHue, uSaturation, uLuminance);
      vec3 rgb = hsl2rgb(hsl);
      
      fragColor = vec4(rgb, color.a);
    }
  `
);
