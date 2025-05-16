import { z } from "zod";
import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";

export const Vec2Def = defineType("vec2", (meta: TypeMeta<[number, number]>) =>
  TypeSpec("vec2", meta, z.tuple([z.number(), z.number()])),
);

export type Vec2Def = ReturnType<typeof Vec2Def>;
export type Vec2 = z.output<Vec2Def["schema"]>;
