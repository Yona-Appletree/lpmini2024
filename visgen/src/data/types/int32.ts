import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeDef } from "../type-def.ts";

export const Int32 = GenericTypeDef("int32", (meta: BaseTypeMeta = {}) =>
  TypeDef("int32", meta, z.number().int().min(-2147483648).max(2147483647)),
);
