import { z } from "zod";
import { FloatParam } from "./params/float-param.ts";
import { Vec4Param } from "./params/vec4-param.ts";
import { Vec2Param } from "./params/vec2-param.ts";
import { Vec3Param } from "./params/vec3-param.ts";
import { IntParam } from "./params/int-param.ts";

export const EffectParam = z.discriminatedUnion("type", [
  IntParam.schema,
  FloatParam.schema,
  Vec2Param.schema,
  Vec3Param.schema,
  Vec4Param.schema,
]);

export type EffectParam = z.infer<typeof EffectParam>;
