import { EffectParamDef } from "../effect-param-def.ts";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";
import { Vec2 } from "../../data/types/vec2.ts";

export const Vec2Param = EffectParamDef("vec2", {
  default: Vec2.schema.default([0, 0]),
  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec2Param = ReturnType<typeof Vec2Param>;
