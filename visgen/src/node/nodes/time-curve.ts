import { NodeDef } from "../node-def.ts";
import { EnumParam } from "../../effect-param/params/enum-param.ts";
import { FloatParam } from "../../effect-param/params/float-param.ts";
import { FloatDef } from "../../type/types/float-def.ts";
import { EnumDef } from "../../type/types/enum-def.ts";

export const TimeCurve = NodeDef("time-curve", {
  label: "Time-based curve",
  input: {
    period: FloatDef({
      default: 5,
      min: 0.1,
      kind: "time",
      unit: "seconds",
    }),
    easing: EnumDef({
      default: "sawtooth",
      options: [
        "sawtooth",
        "sine",
        "square",
        "triangle",
        "exp",
        "exp-in-out",
        "exp-in",
        "exp-out",
      ],
    }),
  },
  output: {
    value: FloatDef(),
  },
});
