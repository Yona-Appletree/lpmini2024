import { EffectParamDef } from "../effect-param-def.ts";
import { z } from "zod";

export const EnumParam = EffectParamDef("enum", {
  default: z.string().default(""),
  options: z.array(z.string()).default([]),
});
export type EnumParam = ReturnType<typeof EnumParam>;
