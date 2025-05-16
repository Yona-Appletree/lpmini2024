import { type BaseTypeMeta, GenericTypeDef, TypeDef } from "../type-def.ts";
import { z } from "zod";

export const Vec4 = GenericTypeDef("vec4", (meta: BaseTypeMeta) =>
  TypeDef("vec4", meta, z.tuple([z.number(), z.number()])),
);

export type Vec4 = ReturnType<typeof Vec4>;
