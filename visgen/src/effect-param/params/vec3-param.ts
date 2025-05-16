import { EffectParamDef } from "../effect-param-def.ts";
import { ScalarQuantity } from "../../data/scalar-quantity.ts";
import { ValueUnit } from "../../data/value-unit.ts";
import { Vec3 } from "../../data/types/vec3.ts";

export const Vec3Param = EffectParamDef("vec3", {
  default: Vec3.schema.default([0, 0, 0]),
  kind: ScalarQuantity.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec3Param = ReturnType<typeof Vec3Param>;
