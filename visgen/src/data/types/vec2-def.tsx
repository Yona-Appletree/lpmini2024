import type { TypeMeta } from "@/data/type-spec.ts";
import { FloatDef } from "@/data/types/float-def.tsx";
import { TupleDef } from "./tuple-def.tsx";
import type { SetOptional } from "type-fest";

export function Vec2Def(meta: SetOptional<TypeMeta<Vec2>, "default">) {
  return TupleDef([FloatDef({ default: 0 }), FloatDef({ default: 0 })], {
    ...meta,
    glType: "vec2",
  });
}
export type Vec2Def = ReturnType<typeof Vec2Def>;
export type Vec2 = [number, number];
