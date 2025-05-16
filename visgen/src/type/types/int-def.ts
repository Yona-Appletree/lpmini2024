import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";

export const IntDef = GenericTypeDef("int32", (meta: BaseTypeMeta = {}) =>
  TypeSpec("int32", meta, z.number().int().min(-2147483648).max(2147483647)),
);
