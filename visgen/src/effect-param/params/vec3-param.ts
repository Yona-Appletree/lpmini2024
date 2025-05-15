import { EffectParamDef } from "../effect-param-def.ts";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";
import { Vec3 } from "../../data/types/vec3.ts";

export const Vec3Param = EffectParamDef("vec3", {
  default: Vec3.schema.default([0, 0, 0]),
  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec3Param = ReturnType<typeof Vec3Param>;
