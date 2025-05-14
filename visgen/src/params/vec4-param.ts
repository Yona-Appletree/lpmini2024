import { BaseEffectParam, EffectParamDef } from "./base-effect-param.ts";
import { z } from "zod";

export const Vec4Data = z.tuple([
  z.number(),
  z.number(),
  z.number(),
  z.number(),
]);
export type Vec4Data = z.infer<typeof Vec4Data>;

export const Vec4DataType = z.enum(["raw", "color:rgba"]);
export type Vec4DataType = z.infer<typeof Vec4DataType>;

export const Vec4Param = EffectParamDef("vec4", {
  default: Vec4Data.default([0, 0, 0, 0]),
  dataType: Vec4DataType.default("raw"),
});
export type Vec4Param = ReturnType<typeof Vec4Param>;
