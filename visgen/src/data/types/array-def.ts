import { z } from "zod";
import {
  defineType,
  type TypeMeta,
  TypeSpec,
  type TypeValue,
} from "../type-spec.ts";

export const ArrayDef = defineType(
  "array",
  <TItem extends TypeSpec>(
    itemType: TItem,
    meta: Omit<TypeMeta<unknown>, "default"> & {
      default?: Array<TypeValue<TItem>>;
    } = {},
  ) =>
    TypeSpec(
      "array",
      { ...meta, default: [], itemType },
      z.array(itemType.schema),
    ),
);
export type ArrayDef = ReturnType<typeof ArrayDef>;

export interface ArrayMeta<TItemSpec extends TypeSpec = TypeSpec>
  extends TypeMeta<TypeValue<TItemSpec>> {
  itemType: TItemSpec;
}
