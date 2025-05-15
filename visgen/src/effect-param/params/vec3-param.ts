import { EffectParamDef } from "../effect-param-def.ts";
import { z } from "zod";
import { ValueKind } from "../value-kind.ts";
import { ValueUnit } from "../value-unit.ts";
import { ZodDef } from "../../util/zod/zod-def.ts";

export const Vec3 = ZodDef(z.tuple([z.number(), z.number(), z.number()]));
export type Vec3 = ReturnType<typeof Vec3>;

export const Vec3Param = EffectParamDef("vec3", {
  default: Vec3.schema.default([0, 0, 0]),
  kind: ValueKind.default("unknown"),
  unit: ValueUnit.default("unknown"),
});
export type Vec3Param = ReturnType<typeof Vec3Param>;
