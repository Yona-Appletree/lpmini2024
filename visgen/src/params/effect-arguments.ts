import { z } from "zod";
import type { EffectParams } from "./effect-params";
import type { EffectParam } from "./effect-param";

export type EffectArguments<T extends EffectParams> = {
  [K in keyof T]: T[K] extends EffectParam ? T[K]["default"] : never;
};
