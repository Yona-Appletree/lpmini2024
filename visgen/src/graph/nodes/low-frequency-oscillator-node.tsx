import { FloatDef } from "../../data/types/float-def.ts";
import { EnumDef } from "../../data/types/enum-def.ts";
import { RecordDef } from "../../data/types/record-def.ts";
import { easingFunctions, easingTypes } from "../../util/easing.ts";
import { defineNode } from "../define-node.ts";

export const LowFrequencyOscillator = defineNode(
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
    return {
      update: ({ input }) => {
        const nowSeconds = Date.now() / 1000;
        const phase = (nowSeconds % input.period) / input.period;
        const value =
          phase < 0.5
            ? easingFunctions[input.easing](phase * 2)
            : easingFunctions[input.easing]((1 - phase) * 2);

        return value * (input.max - input.min) + input.min;
      },
      component: ({ output }) => {
        return <div>value = {output}</div>;
      },
    };
  },
);
