import { z } from "zod";
import {
  type BaseTypeMeta,
  GenericTypeDef,
  type TypeSpec,
} from "../type-spec-fn.ts";

export const ArrayDef = GenericTypeDef(
  "array",
  (itemType: TypeSpec, meta: BaseTypeMeta = {}) =>
    TypeDef("array", { ...meta, itemType }, z.array(itemType.schema)),
);
export type ArrayDef = ReturnType<typeof ArrayDef>;

export interface ArrayMeta<T extends TypeSpec = TypeSpec> extends BaseTypeMeta {
  itemType: T;
}
