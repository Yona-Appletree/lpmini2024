import { z } from "zod";
import { ObjectDef } from "../util/zod/object-def.ts";
import { EffectConfig } from "./effect.ts";
import { Vec2Def } from "../type/types/vec2-def.ts";

export const EffectStackConfig = ObjectDef({
  size: Vec2Def.schema,
  effects: z.array(EffectConfig.schema),
});
export type EffectStackConfig = ReturnType<typeof EffectStackConfig>;
