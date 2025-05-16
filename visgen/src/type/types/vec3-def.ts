import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeDef } from "../type-def.ts";

export const Vec3Def = GenericTypeDef("vec3", (meta: BaseTypeMeta) =>
  TypeDef("vec3", meta, z.tuple([z.number(), z.number()])),
);

export type Vec3 = ReturnType<typeof Vec3Def>;
