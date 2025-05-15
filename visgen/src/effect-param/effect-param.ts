import { FloatParam } from "./params/float-param.ts";
import { Vec4Param } from "./params/vec4-param.ts";
import { Vec2Param } from "./params/vec2-param.ts";
import { Vec3Param } from "./params/vec3-param.ts";
import { IntParam } from "./params/int-param.ts";
import { UnionDef } from "../util/zod/union-def.ts";

export const EffectParam = UnionDef("type", [
  IntParam.schema,
  FloatParam.schema,
  Vec2Param.schema,
  Vec3Param.schema,
  Vec4Param.schema,
]);

export type EffectParam = ReturnType<typeof EffectParam>;
