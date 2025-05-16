import { z } from "zod";
import {
  type BaseTypeMeta,
  GenericTypeDef,
  TypeDef,
  type TypeSpec,
} from "../type-def.ts";

export const ArrayOf = GenericTypeDef(
  "array",
  (itemType: TypeSpec, meta: BaseTypeMeta = {}) =>
    TypeDef("array", { ...meta, itemType }, z.array(itemType.schema)),
);
