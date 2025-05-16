import { z } from "zod";
import { type BaseTypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";

export const ArrayDef = GenericTypeDef(
  "array",
  (itemType: TypeSpec, meta: BaseTypeMeta = {}) =>
    TypeSpec("array", { ...meta, itemType }, z.array(itemType.schema)),
);
export type ArrayDef = ReturnType<typeof ArrayDef>;

export interface ArrayMeta<T extends TypeSpec = TypeSpec> extends BaseTypeMeta {
  itemType: T;
}
