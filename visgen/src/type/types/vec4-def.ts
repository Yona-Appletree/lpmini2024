import { type BaseTypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";
import { z } from "zod";

export const Vec4Def = GenericTypeDef("vec4", (meta: Vec4Meta) =>
  TypeSpec("vec4", meta, z.tuple([z.number(), z.number()])),
);

export type Vec4 = ReturnType<typeof Vec4Def>;

export interface Vec4Meta extends BaseTypeMeta {
  quantity?: {
    type: "color";
    encoding: "normalized";
  };
}
