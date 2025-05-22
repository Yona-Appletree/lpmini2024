import type { TypeMetaInfo } from "@/data/type-spec.ts";
import { FloatDef } from "@/data/types/float-def.tsx";
import { ArrayDef } from "@/data/types/array/array-def.tsx";

export function Vec3Def(meta: TypeMetaInfo) {
  return ArrayDef(FloatDef({ default: 0 }), {
    ...meta,
    defaultLength: 3,
    glType: "vec3",
  });
}
