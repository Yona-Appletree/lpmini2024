import { FloatDef } from "@/core/data/types/float-def.tsx";
import { EnumDef } from "@/core/data/types/enum-def.tsx";
import { RecordDef } from "@/core/data/types/record-def.tsx";
import { defineModule } from "../define-module.ts";
import { TimeSeriesCanvas } from "@/frontend/util/time-series-canvas.ts";
import { CanvasImage } from "@/frontend/components/canvas-image.tsx";
import { AdjustableOscillator } from "@/frontend/util/adjustable-oscillator.ts";
import type { TypeValue } from "@/core/data/type-spec.ts";

const WaveFunctionKey = EnumDef({
  default: "sawtooth",
  options: ["sawtooth", "sine", "square", "triangle"],
});
type WaveFunctionKey = TypeValue<typeof WaveFunctionKey>;

export const OscillatorModule = defineModule(
  "time-curve",
  {
    label: "Oscillator",
    input: RecordDef({
      period: FloatDef({
        default: 5,
        unit: "seconds",
      }),
      easing: WaveFunctionKey,
      min: FloatDef({ default: 0 }),
      max: FloatDef({ default: 1 }),
    }),
    output: FloatDef({ default: 0 }),
  },
  () => {
    const timeSeriesCanvas = TimeSeriesCanvas();
    const oscillator = AdjustableOscillator({ period: 5 });

    return {
      update: ({ input }) => {
        const nowSeconds = Date.now() / 1000;
        oscillator.updatePeriod({
          newPeriod: input.period,
          currentTime: nowSeconds,
        });
        const phase = oscillator.computeValue(nowSeconds);
        const value = waveFunctions[input.easing ?? "sawtooth"](phase);

        timeSeriesCanvas.add(value);

        return value * (input.max - input.min) + input.min;
      },
      component: () => {
        return (
          <div>
            <CanvasImage image={timeSeriesCanvas} />
          </div>
        );
      },
    };
  },
);

const waveFunctions: Record<WaveFunctionKey, (phase: number) => number> = {
  sawtooth: (phase: number) => phase,
  sine: (phase: number) => Math.sin(phase * 2 * Math.PI) * 0.5 + 0.5,
  square: (phase: number) => (phase < 0.5 ? 1 : 0),
  triangle: (phase: number) => {
    return phase < 0.5
      ? 2 * phase // Linear increase from 0 to 1
      : 2 * (1 - phase); // Linear decrease from 1 to 0
  },
};
