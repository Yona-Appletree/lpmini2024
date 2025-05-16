import { z } from "zod";
import { type TypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";

export const Vec2Def = GenericTypeDef(
  "vec2",
  (meta: TypeMeta<[number, number]>) =>
    TypeSpec("vec2", meta, z.tuple([z.number(), z.number()])),
);

export type Vec2 = ReturnType<typeof Vec2Def>;
