import type { TypeMeta } from "@/data/type-spec.ts";
import { FloatDef } from "@/data/types/float-def.tsx";
import { TupleDef } from "./tuple-def.tsx";
import type { SetOptional } from "type-fest";

export function Vec4Def(meta: SetOptional<TypeMeta<Vec4>, "default">) {
  return TupleDef(
    [
      FloatDef({ default: 0 }),
      FloatDef({ default: 0 }),
      FloatDef({ default: 0 }),
      FloatDef({ default: 0 }),
    ],
    {
      ...meta,
      glType: "vec4",
    },
  );
}

export type Vec4 = [number, number, number, number];
