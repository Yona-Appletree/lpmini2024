import { z } from "zod";
import { Float32Param } from "./float32-param";
import { Vec4Param } from "./vec4-param";

export const EffectParam = z.discriminatedUnion("type", [
  Float32Param.schema,
  Vec4Param.schema,
]);

export type EffectParam = z.infer<typeof EffectParam>;
