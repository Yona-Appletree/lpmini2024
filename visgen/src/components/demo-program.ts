import { GlCheckerboardModule } from "@/program/modules/gl-checkerboard-module.tsx";
import { GlPolarScrollNode } from "@/program/modules/gl-polar-scroll-module.tsx";
import { GlRotateNode } from "@/program/modules/gl-rotate-module.tsx";
import { LowFrequencyOscillator } from "@/program/modules/low-frequency-oscillator-module.tsx";
import { ProgramConfig } from "@/program/program-config.ts";

export const demoConfig = ProgramConfig({
  nodes: {
    lfo: LowFrequencyOscillator.Config({
      input: {
        value: {
          period: { value: 5 },
          easing: { value: "quadInOut" },
          min: { value: 0 },
          max: { value: 1 },
        },
      },
    }),
    lfo2: LowFrequencyOscillator.Config({
      input: {
        value: {
          period: { value: 7 },
          easing: { value: "quadInOut" },
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
                value: [
                  { value: 1 },
                  { value: 0.5 },
                  { value: 0 },
                  { value: 1 },
                ],
              },
              color2: {
                value: [
                  { value: 0 },
                  { value: 0 },
                  { value: 0.5 },
                  { value: 1 },
                ],
              },
              rows: { value: 10 },
              columns: { value: 10 },
            },
          },
        },
      },
    }),

    rotate: GlRotateNode.Config({
      input: {
        value: {
          image: {
            $moduleOutput: {
              moduleId: "checkerboard",
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

    polarScroll: GlPolarScrollNode.Config({
      input: {
        value: {
          image: {
            $moduleOutput: {
              moduleId: "rotate",
            },
            activeExpr: "$moduleOutput",
          },
          args: {
            value: {
              offset: {
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
