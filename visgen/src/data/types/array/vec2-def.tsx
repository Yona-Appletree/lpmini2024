import type { TypeMetaInfo } from "@/data/type-spec.ts";
import { FloatDef } from "@/data/types/float-def.tsx";
import { ArrayDef } from "@/data/types/array/array-def.tsx";

export function Vec2Def(meta: TypeMetaInfo) {
  return ArrayDef(FloatDef({ default: 0 }), {
    ...meta,
    defaultLength: 2,
    glType: "vec2",
  });
}
