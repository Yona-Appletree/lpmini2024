import { z } from "zod";
import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";

export const IntDef = defineType("int32", (meta: TypeMeta<number>) =>
  TypeSpec("int32", meta, z.number().int().min(-2147483648).max(2147483647)),
);
