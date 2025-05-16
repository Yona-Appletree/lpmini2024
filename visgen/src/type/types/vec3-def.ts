import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";

export const Vec3Def = GenericTypeDef("vec3", (meta: BaseTypeMeta) =>
  TypeSpec("vec3", meta, z.tuple([z.number(), z.number()])),
);

export type Vec3 = ReturnType<typeof Vec3Def>;
