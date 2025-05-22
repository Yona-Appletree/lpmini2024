import { z } from "zod";
import {
  defineType,
  type TypeMeta,
  TypeSpec,
  type TypeSpecOf,
  type TypeValue,
} from "../type-spec.ts";
import { deepClone } from "@/util/deep-clone.ts";

export const ArrayDef = defineType(
  "array",
  <TItem extends TypeSpec>(
    itemType: TItem,
    meta: Omit<ArrayMeta<TItem>, "default" | "itemType"> & {
      default?: Array<TypeValue<TItem>>;
    } = {},
  ) =>
    TypeSpec(
      "array",
      {
        default: Array.from({ length: meta.defaultLength ?? 0 }).map(() =>
          deepClone(itemType.info.meta.default),
        ),
        ...meta,
        itemType,
      },
      z.array(itemType.schema),
      (props) => {
        const ItemComponent = itemType.component;

        return (
          <div>
            {props.currentValue.map((_, index) => {
              return (
                <div key={index}>
                  <ItemComponent
                    meta={itemType.info.meta}
                    currentValue={props.currentValue[index]}
                    onChange={(value) => {
                      props.onChange(
                        props.currentValue.map((v, i) =>
                          i === index ? value : v,
                        ),
                      );
                    }}
                  />
                </div>
              );
            })}
          </div>
        );
      },
    ),
);
export type ArrayDef = ReturnType<typeof ArrayDef>;
export type ArraySpec = TypeSpecOf<typeof ArrayDef>;

export interface ArrayMeta<TItem extends TypeSpec = TypeSpec>
  extends TypeMeta<Array<TypeValue<TItem>>> {
  itemType: TItem;

  glType?: "vec2" | "vec3" | "vec4";
  defaultLength?: number;
}
