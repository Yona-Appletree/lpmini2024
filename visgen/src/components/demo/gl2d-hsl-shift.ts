import type { Gl2dContext } from "../../gl2d/gl2d-context";
import { Gl2dFragmentShader } from "../../gl2d/gl2d-fragment-shader";

export function Gl2dHslShift(context: Gl2dContext) {
  const shader = Gl2dFragmentShader(context, glsl);

  return {
    draw(hueShift: number, satShift: number, lightShift: number) {
      shader.draw({
        uHueShift: { type: "float", value: hueShift },
        uSatShift: { type: "float", value: satShift },
        uLightShift: { type: "float", value: lightShift },
      });
    },
    [Symbol.dispose]() {
      shader[Symbol.dispose]();
    },
  };
}

export type Gl2dHslShift = ReturnType<typeof Gl2dHslShift>;

const glsl = `
  #version 300 es
  precision highp float;

  in vec2 vUv;
  out vec4 fragColor;
  uniform sampler2D uInputTexture;
  uniform float uHueShift;
  uniform float uSatShift;
  uniform float uLightShift;

  // Helper functions for RGB <-> HSL
  vec3 rgb2hsl(vec3 c) {
    float maxC = max(max(c.r, c.g), c.b);
    float minC = min(min(c.r, c.g), c.b);
    float l = (maxC + minC) * 0.5;
    float s = 0.0;
    float h = 0.0;
    if (maxC != minC) {
      float d = maxC - minC;
      s = l > 0.5 ? d / (2.0 - maxC - minC) : d / (maxC + minC);
      if (maxC == c.r) {
        h = (c.g - c.b) / d + (c.g < c.b ? 6.0 : 0.0);
      } else if (maxC == c.g) {
        h = (c.b - c.r) / d + 2.0;
      } else {
        h = (c.r - c.g) / d + 4.0;
      }
      h /= 6.0;
    }
    return vec3(h, s, l);
  }

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
    vec3 hsl = rgb2hsl(color.rgb);
    hsl.x = fract(hsl.x + uHueShift); // Hue wraps
    hsl.y = clamp(hsl.y + uSatShift, 0.0, 1.0); // Clamp saturation
    hsl.z = clamp(hsl.z + uLightShift, 0.0, 1.0); // Clamp lightness
    vec3 rgb = hsl2rgb(hsl);
    fragColor = vec4(rgb, color.a);
  }
`.trim();
