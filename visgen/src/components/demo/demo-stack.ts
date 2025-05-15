import { EffectStackConfig } from "../../effect-stack/effect-stack-config.ts";
import { Checkerboard } from "../../effect-stack/effects/checkerboard.ts";

export const demoStack = EffectStackConfig({
  size: [128, 128],
  effects: [
    Checkerboard.Config({
      args: {
        color1: [1, 0.5, 0, 1],
        color2: [0, 0, 0.5, 1],
        rows: 10,
        columns: 10,
      },
    }),
  ],
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
