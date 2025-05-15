import { EffectParamDef } from "../effect-param-def.ts";
import { z } from "zod";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";
import { ZodDef } from "../../util/zod/zod-def.ts";

export const Vec4 = ZodDef(
  z.tuple([z.number(), z.number(), z.number(), z.number()]),
);
export type Vec4 = ReturnType<typeof Vec4>;

export const Vec4Param = EffectParamDef("vec4", {
  default: Vec4.schema.default([0, 0, 0, 0]),
  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec4Param = ReturnType<typeof Vec4Param>;
