import { type BaseTypeMeta, GenericTypeDef, TypeDef } from "../type-def.ts";
import { z } from "zod";

export const Vec4Def = GenericTypeDef("vec4", (meta: Vec4Meta) =>
  TypeDef("vec4", meta, z.tuple([z.number(), z.number()])),
);

export type Vec4 = ReturnType<typeof Vec4Def>;

export interface Vec4Meta extends BaseTypeMeta {
  quantity?: {
    type: "color";
    encoding: "normalized";
  };
}
