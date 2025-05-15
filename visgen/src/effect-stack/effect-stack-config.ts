import { z } from "zod";
import { ObjectDef } from "../util/zod/object-def.ts";
import { EffectConfig } from "./effect.ts";
import { Vec2 } from "../data/types/vec2.ts";

export const EffectStackConfig = ObjectDef({
  size: Vec2.schema,
  effects: z.array(EffectConfig.schema),
});
export type EffectStackConfig = ReturnType<typeof EffectStackConfig>;
