import { NodeDef } from "../node-def.ts";
import { FloatDef } from "../../type/types/float-def.ts";
import { EnumDef } from "../../type/types/enum-def.ts";
import { RecordDef } from "../../type/types/record-def.ts";
import { easingTypes } from "../../util/easing.ts";

export const LowFrequencyOscillator = NodeDef(
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
        return input.period;
      },
    };
  },
);
