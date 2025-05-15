import { z } from "zod";
import { ObjectDef } from "../util/zod/object-def.ts";
import { Vec2 } from "../effect-param/params/vec2-param.ts";
import { EffectConfig } from "./effect.ts";

export const EffectStackConfig = ObjectDef({
  size: Vec2.schema,
  effects: z.array(EffectConfig.schema),
});
export type EffectStackConfig = ReturnType<typeof EffectStackConfig>;
