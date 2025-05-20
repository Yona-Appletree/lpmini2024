import { z } from "zod";
import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";

export const Vec3Def = defineType(
  "vec3",
  (meta: TypeMeta<[number, number, number]>) =>
    TypeSpec("vec3", meta, z.tuple([z.number(), z.number(), z.number()])),
);

export type Vec3Def = ReturnType<typeof Vec3Def>;
export type Vec3 = z.output<Vec3Def["schema"]>;
