import { EffectParamDef } from "../effect-param-def.ts";
import { ScalarQuantity } from "../../data/scalar-quantity.ts";
import { ValueUnit } from "../../data/value-unit.ts";

export const IntParam = EffectParamDef("int", {
  default: IntParam.schema.default(0),
  min: IntParam.schema.optional(),
  max: IntParam.schema.optional(),
  step: IntParam.schema.default(1),

  kind: ScalarQuantity.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type IntParam = ReturnType<typeof IntParam>;
