import { EffectParamDef } from "../base-effect-param.ts";
import { z } from "zod";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";

export const Vec2 = z.tuple([z.number(), z.number()]);
export type Vec2 = z.infer<typeof Vec2>;

export const Vec2Param = EffectParamDef("vec2", {
  default: Vec2.default([0, 0]),
  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec2Param = ReturnType<typeof Vec2Param>;
