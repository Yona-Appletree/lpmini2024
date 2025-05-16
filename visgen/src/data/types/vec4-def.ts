import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";
import { z } from "zod";

export const Vec4Def = defineType("vec4", (meta: Vec4Meta) =>
  TypeSpec(
    "vec4",
    meta,
    z.tuple([z.number(), z.number(), z.number(), z.number()]),
  ),
);

export type Vec4Def = ReturnType<typeof Vec4Def>;
export type Vec4 = z.output<Vec4Def["schema"]>;

export interface Vec4Meta extends TypeMeta<[number, number, number, number]> {
  quantity?: {
    type: "color";
    encoding: "normalized";
  };
}
