import { EffectParamDef } from "../effect-param-def.ts";
import { z } from "zod";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";
import { ZodDef } from "../../util/zod/zod-def.ts";

export const Vec2 = ZodDef(z.tuple([z.number(), z.number()]));
export type Vec2 = ReturnType<typeof Vec2>;

export const Vec2Param = EffectParamDef("vec2", {
  default: Vec2.schema.default([0, 0]),
  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec2Param = ReturnType<typeof Vec2Param>;
