import { GlCheckerboardModule } from "@/core/program/modules/gl-checkerboard-module.tsx";
import { GlMonoToHueModule } from "@/core/program/modules/gl-mono-to-hue-module.tsx";
import { GlPerlinModule } from "@/core/program/modules/gl-perlin-module.tsx";
import { GlPolarScrollNode } from "@/core/program/modules/gl-polar-scroll-module.tsx";
import { GlRotateNode } from "@/core/program/modules/gl-rotate-module.tsx";
import { OscillatorModule } from "@/core/program/modules/oscillator-module.tsx";
import { ProgramConfig } from "@/core/program/program-config.ts";

export const demoConfig = ProgramConfig({
  nodes: {
    lfo2: OscillatorModule.Config({
      input: {
        value: {
          period: { value: 7 },
          easing: { value: "sawtooth" },
          min: { value: 0 },
          max: { value: 1 },
        },
      },
    }),

    checkerboard: GlCheckerboardModule.Config({
      input: {
        value: {
          color1: {
            $hexColor: "#000000",
            activeExpr: "$hexColor",
          },
          color2: {
            $hexColor: "#FFFFFF",
            activeExpr: "$hexColor",
          },
          rows: { value: 5 },
          columns: { value: 5 },
        },
      },
    }),

    lfo3: OscillatorModule.Config({
      input: {
        value: {
          period: { value: 300 },
          easing: { value: "triangle" },
          min: { value: 0 },
          max: { value: 50 },
        },
      },
    }),

    perlin: GlPerlinModule.Config({
      input: {
        value: {
          color: {
            $hexColor: "#FFFFFF",
            activeExpr: "$hexColor",
          },
          scale: { value: 1 },
          timeOffset: {
            $moduleOutput: {
              moduleId: "lfo3",
            },
            activeExpr: "$moduleOutput",
          },
          octaves: { value: 3 },
          persistence: { value: 0.4 },
          lacunarity: { value: 2 },
          contrast: { value: 1.1 },
          brightness: { value: 0.01 },
          sharpness: { value: 15 },
        },
      },
    }),

    monoToHue: GlMonoToHueModule.Config({
      input: {
        value: {
          inputTexture: {
            $moduleOutput: {
              moduleId: "perlin",
            },
            activeExpr: "$moduleOutput",
          },
          saturation: { value: 0.5 },
          luminance: { value: 0.5 },
          hueShift: {
            $moduleOutput: {
              moduleId: "lfo",
            },
            activeExpr: "$moduleOutput",
          },
          compressionFactor: { value: 0.2 },
          compressionFeather: { value: 0.15 },
        },
      },
    }),

    polarScroll: GlPolarScrollNode.Config({
      input: {
        value: {
          inputTexture: {
            $moduleOutput: {
              moduleId: "monoToHue",
            },
            activeExpr: "$moduleOutput",
          },
          offset: {
            $moduleOutput: {
              moduleId: "lfo2",
            },
            activeExpr: "$moduleOutput",
          },
        },
      },
    }),

    lfo: OscillatorModule.Config({
      input: {
        value: {
          period: { value: 5 },
          easing: { value: "sine" },
          min: { value: -0.25 },
          max: { value: 0.25 },
        },
      },
    }),
    rotate: GlRotateNode.Config({
      input: {
        value: {
          inputTexture: {
            $moduleOutput: {
              moduleId: "polarScroll",
            },
            activeExpr: "$moduleOutput",
          },
          angle: { value: 0 },
          swirl: {
            $moduleOutput: {
              moduleId: "lfo",
            },
            activeExpr: "$moduleOutput",
          },
        },
      },
    }),
  },
});
