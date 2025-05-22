import type { TypeSpec, TypeValue } from "../data/type-spec.ts";
import type { TypeName } from "../data/type-def.ts";
import type { ArrayMeta } from "../data/types/array-def.tsx";
import { type RecordMeta } from "../data/types/record-def.tsx";
import { z } from "zod";
import { mapValues } from "../util/map-values.ts";
import { ConfigExprKey, ConfigNodeExpr } from "./config-node.ts";
import type { TupleItems, TupleMeta } from "@/data/types/tuple-def.tsx";

export function configSchemaFor<TSpec extends TypeSpec<TypeName>>(
  spec: TSpec,
): z.Schema<ConfigValueFor<TSpec>> {
  const valueSchema = (() => {
    switch (spec.info.name) {
      case "array":
        return z.array(configSchemaFor((spec.info.meta as ArrayMeta).itemType));

      case "tuple":
        return z.tuple(
          (spec.info.meta as TupleMeta).itemTypes.map((it: unknown) =>
            configSchemaFor(it as TypeSpec<TypeName>),
          ),
        );

      case "record":
        return z.object(
          mapValues((spec.info.meta as RecordMeta).shape, (type) =>
            configSchemaFor(type),
          ),
        );

      default:
        return spec.schema;
    }
  })();

  return z.object({
    ...ConfigNodeExpr.schema.shape,
    value: valueSchema.optional(),
    activeExpr: ConfigExprKey.schema.optional(),
  }) as unknown as z.Schema<ConfigValueFor<TSpec>>;
}

export type ConfigValueFor<T extends TypeSpec<TypeName> = TypeSpec<TypeName>> =
  Partial<(typeof ConfigNodeExpr)["schema"]["shape"]> & {
    activeExpr?: ConfigExprKey;
    value?: // Array
    T["info"]["meta"] extends ArrayMeta
      ? Array<ConfigValueFor<T["info"]["meta"]["itemType"]>>
      : // Record
        T["info"]["meta"] extends RecordMeta
        ? {
            [TKey in keyof T["info"]["meta"]["shape"]]: ConfigValueFor<
              T["info"]["meta"]["shape"][TKey]
            >;
          }
        : // Tuple
          T["info"]["meta"] extends TupleMeta
          ? ConfigValueForTuple<T["info"]["meta"]["itemTypes"]>
          : // everything else
            TypeValue<T>;
  };

export type ConfigValueForTuple<T extends TupleItems> = T extends [
  infer First,
  ...infer Rest,
]
  ? First extends TypeSpec
    ? Rest extends TupleItems
      ? [ConfigValueFor<First>, ...ConfigValueForTuple<Rest>]
      : [ConfigValueFor<First>]
    : []
  : [];
