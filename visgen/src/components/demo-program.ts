import { GlCheckerboardModule } from "@/program/modules/gl-checkerboard-module.tsx";
import { GlMonoToHueModule } from "@/program/modules/gl-mono-to-hue-module";
import { GlPerlinModule } from "@/program/modules/gl-perlin-module";
import { GlPolarScrollNode } from "@/program/modules/gl-polar-scroll-module.tsx";
import { GlRotateNode } from "@/program/modules/gl-rotate-module.tsx";
import { OscillatorModule } from "@/program/modules/oscillator-module.tsx";
import { ProgramConfig } from "@/program/program-config.ts";

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
          image: { value: null },
          args: {
            value: {
              color1: {
                $hexColor: "#32A5A3",
                activeExpr: "$hexColor",
              },
              color2: {
                $hexColor: "#780011",
                activeExpr: "$hexColor",
              },
              rows: { value: 10 },
              columns: { value: 10 },
            },
          },
        },
      },
    }),

    lfo3: OscillatorModule.Config({
      input: {
        value: {
          period: { value: 30 },
          easing: { value: "triangle" },
          min: { value: 0 },
          max: { value: 5 },
        },
      },
    }),

    perlin: GlPerlinModule.Config({
      input: {
        value: {
          image: { value: null },
          args: {
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
        },
      },
    }),

    monoToHue: GlMonoToHueModule.Config({
      input: {
        value: {
          image: {
            $moduleOutput: {
              moduleId: "perlin",
            },
            activeExpr: "$moduleOutput",
          },
          args: {
            value: {
              saturation: { value: 0.5 },
              luminance: { value: 0.5 },
            },
          },
        },
      },
    }),

    polarScroll: GlPolarScrollNode.Config({
      input: {
        value: {
          image: {
            $moduleOutput: {
              moduleId: "monoToHue",
            },
            activeExpr: "$moduleOutput",
          },
          args: {
            value: {
              offset: {
                $moduleOutput: {
                  moduleId: "lfo2",
                },
                activeExpr: "$moduleOutput",
              },
            },
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
          image: {
            $moduleOutput: {
              moduleId: "polarScroll",
            },
            activeExpr: "$moduleOutput",
          },
          args: {
            value: {
              angle: { value: 0 },
              swirl: {
                $moduleOutput: {
                  moduleId: "lfo",
                },
                activeExpr: "$moduleOutput",
              },
            },
          },
        },
      },
    }),
  },
});

/*

      // Draw checkerboard
      gl2d.ops.checkerboard.draw([1, 0.5, 0, 1], [0, 0, 0.5, 1]);

      // Rotate
      gl2d.ops.rotate.draw(0, fracTimeCosine(6000, { min: 0.5, max: 1 }));

      // Apply polar scroll with animated offset
      gl2d.ops.polarScroll.draw(fracTimeSawtooth(2500));

      // Blur
      gl2d.ops.blur.draw(25, 0.25);

      // Apply HSL shift
      gl2d.ops.hslShift.draw(fracTimeCosine(3500), 0, 0);
      */
