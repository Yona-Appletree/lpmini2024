import { EffectParamDef } from "./base-effect-param.ts";
import { z } from "zod";

export const Float32Param = EffectParamDef("float32", {
  default: z.number().default(0),
  min: z.number().optional(),
  max: z.number().optional(),
  step: z.number().default(0.1),
});
export type Float32Param = ReturnType<typeof Float32Param>;
