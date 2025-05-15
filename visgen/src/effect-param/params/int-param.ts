import { EffectParamDef } from "../effect-param-def.ts";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";
import { Int32 } from "../../util/types/int32.ts";

export const IntParam = EffectParamDef("int", {
  default: Int32.schema.default(0),
  min: Int32.schema.optional(),
  max: Int32.schema.optional(),
  step: Int32.schema.default(1),

  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type IntParam = ReturnType<typeof IntParam>;
