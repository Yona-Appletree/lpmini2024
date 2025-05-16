import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";

export const FloatDef = GenericTypeDef("float32", (meta: BaseTypeMeta = {}) =>
  TypeSpec("float32", meta, z.number().int().min(-2147483648).max(2147483647)),
);
