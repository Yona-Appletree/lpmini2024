import { EffectParamDef } from "../effect-param-def.ts";
import { ScalarQuantity } from "../../type/scalar-quantity.ts";
import { ValueUnit } from "../../type/value-unit.ts";
import { Vec2Def } from "../../type/types/vec2-def.ts";

export const Vec2Param = EffectParamDef("vec2", {
  default: Vec2Def.schema.default([0, 0]),
  kind: ScalarQuantity.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec2Param = ReturnType<typeof Vec2Param>;
