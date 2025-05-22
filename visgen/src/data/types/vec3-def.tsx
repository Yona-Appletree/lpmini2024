import type { TypeMeta } from "@/data/type-spec.ts";
import { FloatDef } from "@/data/types/float-def.tsx";
import { TupleDef } from "./tuple-def.tsx";
import type { SetOptional } from "type-fest";

export function Vec3Def(meta: SetOptional<TypeMeta<Vec3>, "default">) {
  return TupleDef(
    [
      FloatDef({ default: 0 }),
      FloatDef({ default: 0 }),
      FloatDef({ default: 0 }),
    ],
    {
      ...meta,
      glType: "vec3",
    },
  );
}

export type Vec3 = [number, number, number];
