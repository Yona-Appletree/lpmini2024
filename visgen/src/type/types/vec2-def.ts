import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeDef } from "../type-def.ts";

export const Vec2Def = GenericTypeDef("vec2", (meta: BaseTypeMeta) =>
  TypeDef("vec2", meta, z.tuple([z.number(), z.number()])),
);

export type Vec2 = ReturnType<typeof Vec2Def>;
