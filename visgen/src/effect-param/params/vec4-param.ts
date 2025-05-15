import { EffectParamDef } from "../effect-param-def.ts";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";
import { Vec4 } from "../../data/types/vec4.ts";

export const Vec4Param = EffectParamDef("vec4", {
  default: Vec4.schema.default([0, 0, 0, 0]),
  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec4Param = ReturnType<typeof Vec4Param>;
