import { EffectParamDef } from "../effect-param-def.ts";
import { z } from "zod";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";

export const FloatParam = EffectParamDef("float", {
  default: z.number().default(0),
  min: z.number().optional(),
  max: z.number().optional(),
  step: z.number().default(0.1),

  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type FloatParam = ReturnType<typeof FloatParam>;
