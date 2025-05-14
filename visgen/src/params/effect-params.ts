import { z } from "zod";
import { EffectParam } from "./effect-param";

export const EffectParams = z.record(z.string(), EffectParam);
export type EffectParams = Record<string, EffectParam>;
