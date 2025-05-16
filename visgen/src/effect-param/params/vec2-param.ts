import { EffectParamDef } from "../effect-param-def.ts";
import { ScalarQuantity } from "../../data/scalar-quantity.ts";
import { ValueUnit } from "../../data/value-unit.ts";
import { Vec2 } from "../../data/types/vec2.ts";

export const Vec2Param = EffectParamDef("vec2", {
  default: Vec2.schema.default([0, 0]),
  kind: ScalarQuantity.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec2Param = ReturnType<typeof Vec2Param>;
