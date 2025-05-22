import { z } from "zod";
import {
  defineType,
  type TypeMeta,
  TypeSpec,
  type TypeSpecOf,
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
      z.tuple(
        itemTypes.map((it) => it.schema) as TupleSchemas<TItems>,
        // this works around a weird issue with the generic type
        // when the tuple value isn't known
      ) as TItems extends unknown[]
        ? z.ZodTuple<TupleSchemas<TItems>>
        : z.ZodTuple<[]>,
      (props) => (
        <div className="flex flex-wrap gap-2">
          {itemTypes.map((it, index) => {
            const ItemComponent = it.component;

            return (
              <ItemComponent
                key={index}
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
export type TupleSpec = TypeSpecOf<typeof TupleDef>;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export interface TupleMeta<TItems extends TupleItems = any>
  extends TypeMeta<TupleValue<TItems>> {
  itemTypes: TItems;
  glType?: "vec2" | "vec3" | "vec4";
}

export type TupleItems = [] | [TypeSpec, ...TypeSpec[]];
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
