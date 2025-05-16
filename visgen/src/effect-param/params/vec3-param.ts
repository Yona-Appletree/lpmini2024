import { EffectParamDef } from "../effect-param-def.ts";
import { ScalarQuantity } from "../../type/scalar-quantity.ts";
import { ValueUnit } from "../../type/value-unit.ts";
import { Vec3Def } from "../../type/types/vec3-def.ts";

export const Vec3Param = EffectParamDef("vec3", {
  default: Vec3Def.schema.default([0, 0, 0]),
  kind: ScalarQuantity.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec3Param = ReturnType<typeof Vec3Param>;
