import { FloatDef } from "../../data/types/float-def.tsx";
import { EnumDef } from "../../data/types/enum-def.tsx";
import { RecordDef } from "../../data/types/record-def.tsx";
import { easingFunctions, easingTypes } from "../../util/easing.ts";
import { defineModule } from "../define-module.ts";
import { TimeSeriesCanvas } from "@/lib/time-series-canvas.ts";
import { CanvasImage } from "@/components/canvas-image.tsx";

export const LowFrequencyOscillator = defineModule(
  "time-curve",
  {
    label: "Time-based curve",
    input: RecordDef({
      period: FloatDef({
        default: 5,
        unit: "seconds",
      }),
      easing: EnumDef({
        default: "linear",
        options: [...easingTypes],
      }),
      min: FloatDef({ default: 0 }),
      max: FloatDef({ default: 1 }),
    }),
    output: FloatDef({ default: 0 }),
  },
  () => {
    const timeSeriesCanvas = TimeSeriesCanvas();

    return {
      update: ({ input }) => {
        const nowSeconds = Date.now() / 1000;
        const phase = (nowSeconds % input.period) / input.period;
        const value =
          phase < 0.5
            ? easingFunctions[input.easing](phase * 2)
            : easingFunctions[input.easing]((1 - phase) * 2);

        timeSeriesCanvas.add(value);

        return value * (input.max - input.min) + input.min;
      },
      component: ({ output }) => {
        return (
          <div>
            <CanvasImage image={timeSeriesCanvas} />
          </div>
        );
      },
    };
  },
);
