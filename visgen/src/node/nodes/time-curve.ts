import { NodeDef } from "../node-def.ts";
import { EnumParam } from "../../effect-param/params/enum-param.ts";
import { FloatParam } from "../../effect-param/params/float-param.ts";

export const TimeCurve = NodeDef("time-curve", {
  label: "Time-based curve",
  params: {
    period: FloatParam({
      default: 5,
      min: 0.1,
      kind: "time",
      unit: "seconds",
    }),
    easing: EnumParam({
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
});
