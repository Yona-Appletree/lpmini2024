import { z } from "zod";
import { EffectParam } from "./effect-param";

export const EffectParams = z.record(z.string(), EffectParam.schema);
export type EffectParams = Record<string, EffectParam>;
