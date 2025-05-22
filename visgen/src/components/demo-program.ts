import { GlCheckerboardModule } from "@/program/modules/gl-checkerboard-module.tsx";
import { GlPolarScrollNode } from "@/program/modules/gl-polar-scroll-module.tsx";
import { GlRotateNode } from "@/program/modules/gl-rotate-module.tsx";
import { LowFrequencyOscillator } from "@/program/modules/low-frequency-oscillator-module.tsx";
import { ProgramConfig } from "@/program/program-config.ts";

export const demoConfig = ProgramConfig({
  nodes: {
    lfo: LowFrequencyOscillator.Config({
      input: {
        period: 5,
        easing: "quadInOut",
        min: 0,
        max: 1,
      },
    }),

    checkerboard: GlCheckerboardModule.Config({
      input: {
        image: null,
        args: {
          color1: [1, 0.5, 0, 1],
          color2: [0, 0, 0.5, 1],
          rows: 10,
          columns: 10,
        },
      },
    }),

    rotate: GlRotateNode.Config({
      input: {
        image: {
          $expr: "node-output",
          moduleId: "checkerboard",
        },
        args: {
          angle: {
            $expr: "node-output",
            moduleId: "lfo",
          },
          swirl: {
            $expr: "node-output",
            moduleId: "lfo",
          },
        },
      },
    }),

    polarScroll: GlPolarScrollNode.Config({
      input: {
        image: {
          $expr: "node-output",
          moduleId: "rotate",
        },
        args: {
          offset: {
            $expr: "node-output",
            moduleId: "lfo",
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
