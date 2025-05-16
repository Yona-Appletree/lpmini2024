import { z } from "zod";
import { type TypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";

export const Vec3Def = GenericTypeDef(
  "vec3",
  (meta: TypeMeta<[number, number, number]>) =>
    TypeSpec("vec3", meta, z.tuple([z.number(), z.number(), z.number()])),
);

export type Vec3 = ReturnType<typeof Vec3Def>;
