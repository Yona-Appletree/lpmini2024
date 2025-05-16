import { EffectParamDef } from "../effect-param-def.ts";
import { ScalarQuantity } from "../../type/scalar-quantity.ts";
import { ValueUnit } from "../../type/value-unit.ts";
import { Vec4Def } from "../../type/types/vec4-def.ts";

export const Vec4Param = EffectParamDef("vec4", {
  default: Vec4Def.schema.default([0, 0, 0, 0]),
  kind: ScalarQuantity.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec4Param = ReturnType<typeof Vec4Param>;
