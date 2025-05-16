import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";

export const Vec2Def = GenericTypeDef("vec2", (meta: BaseTypeMeta) =>
  TypeSpec("vec2", meta, z.tuple([z.number(), z.number()])),
);

export type Vec2 = ReturnType<typeof Vec2Def>;
