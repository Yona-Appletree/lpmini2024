import { z } from "zod";
import {
  defineType,
  type TypeMeta,
  TypeSpec,
  type TypeValue,
} from "../type-spec.ts";
import { deepClone } from "@/util/deep-clone.ts";
import type { SetOptional } from "type-fest";

export const TupleDef = defineType(
  "tuple",
  <TItems extends TupleItems>(
    itemTypes: TItems,
    meta: SetOptional<Omit<TupleMeta<TItems>, "itemTypes">, "default"> = {},
  ) =>
    TypeSpec(
      "tuple",
      {
        default: itemTypes.map((it) =>
          deepClone(it.info.meta.default),
        ) as TupleValue<TItems>,
        ...meta,
        itemTypes,
      },
      z.tuple(itemTypes.map((it) => it.schema) as TupleSchemas<TItems>),
      (props) => (
        <div className="flex flex-wrap gap-2">
          {itemTypes.map((it, index) => {
            const ItemComponent = it.component;

            return (
              <ItemComponent
                key={index}
                context={props.context}
                meta={it.info.meta}
                currentValue={props.currentValue[index]}
                onChange={(value) => {
                  props.onChange(
                    props.currentValue.map((v, i) =>
                      i === index ? value : v,
                    ) as TupleValue<TItems>,
                  );
                }}
              />
            );
          })}
        </div>
      ),
    ),
);
export type TupleDef = ReturnType<typeof TupleDef>;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export interface TupleMeta<TItems extends TupleItems = any>
  extends TypeMeta<TupleValue<TItems>> {
  itemTypes: TItems;
  glType?: "vec2" | "vec3" | "vec4";
}

export type TupleItems = [TypeSpec, ...TypeSpec[]];
export type TupleValue<T extends TupleItems> = T extends [
  infer First,
  ...infer Rest,
]
  ? First extends TypeSpec
    ? Rest extends TupleItems
      ? [TypeValue<First>, ...TupleValue<Rest>]
      : [TypeValue<First>]
    : []
  : [];

export type TupleSchemas<T extends TupleItems> = T extends [
  infer First,
  ...infer Rest,
]
  ? First extends TypeSpec
    ? Rest extends TupleItems
      ? [First["schema"], ...TupleValue<Rest>]
      : [First["schema"]]
    : []
  : [];
