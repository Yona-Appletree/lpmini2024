import { GraphConfig } from "../../graph/graph-config.ts";
import { GlCheckerboardNode } from "../../graph/nodes/gl-checkerboard-node.tsx";
import { GlPolarScrollNode } from "../../graph/nodes/gl-polar-scroll.tsx";
import { GlRotateNode } from "../../graph/nodes/gl-rotate.tsx";
import { LowFrequencyOscillator } from "../../graph/nodes/low-frequency-oscillator-node.tsx";

export const demoConfig = GraphConfig({
  nodes: {
    lfo: LowFrequencyOscillator.Config({
      input: {
        period: 5,
        easing: "quadInOut",
        min: 0,
        max: 1,
      },
    }),

    checkerboard: GlCheckerboardNode.Config({
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
          nodeId: "checkerboard",
        },
        args: {
          angle: {
            $expr: "node-output",
            nodeId: "lfo",
          },
          swirl: {
            $expr: "node-output",
            nodeId: "lfo",
          },
        },
      },
    }),

    polarScroll: GlPolarScrollNode.Config({
      input: {
        image: {
          $expr: "node-output",
          nodeId: "rotate",
        },
        args: {
          offset: {
            $expr: "node-output",
            nodeId: "lfo",
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
