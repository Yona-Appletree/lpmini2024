import { EffectParamDef } from "../effect-param-def.ts";
import { ScalarQuantity } from "../../data/scalar-quantity.ts";
import { ValueUnit } from "../../data/value-unit.ts";
import { Int32 } from "../../data/types/int32.ts";

export const IntParam = EffectParamDef("int", {
  default: Int32.schema.default(0),
  min: Int32.schema.optional(),
  max: Int32.schema.optional(),
  step: Int32.schema.default(1),

  kind: ScalarQuantity.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type IntParam = ReturnType<typeof IntParam>;
