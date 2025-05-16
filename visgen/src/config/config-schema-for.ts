import type { TypeSpec } from "../type/type-spec.ts";
import type { TypeName } from "../type/type-def.ts";
import type { ArrayDef, ArrayMeta } from "../type/types/array-def.ts";
import { RecordDef, type RecordMeta } from "../type/types/record-def.ts";
import { NodeOutputValue } from "./values/node-value.ts";
import { z } from "zod";
import { mapValues } from "../util/map-values.ts";

export function configSchemaFor<TSpec extends TypeSpec<TypeName>>(
  spec: TSpec
): ConfigSchemaFor<TSpec> {
  switch (spec.info.name) {
    case "array":
      return z.array(
        configSchemaFor((spec.info.meta as ArrayMeta).itemType)
      ) as ConfigSchemaFor<TSpec>;
    case "record":
      return z.object(
        mapValues((spec.info.meta as RecordMeta).shape, (type) =>
          configSchemaFor(type)
        )
      ) as ConfigSchemaFor<TSpec>;
    default:
      return z.union([
        spec.schema,
        NodeOutputValue.schema,
      ]) as ConfigSchemaFor<TSpec>;
  }
}

export type ConfigSchemaFor<T extends TypeSpec<TypeName>> =
  // array
  T["info"] extends ArrayDef
    ? T["info"]["meta"] extends ArrayMeta
      ? z.ZodArray<ConfigSchemaFor<T["info"]["meta"]["itemType"]>>
      : never
    : // record
      T["info"] extends RecordDef
      ? T["info"]["meta"] extends RecordMeta
        ? z.ZodObject<{
            [TKey in keyof T["info"]["meta"]["shape"]]: ConfigSchemaFor<
              T["info"]["meta"]["shape"][TKey]
            >;
          }>
        : never
      : // everything else
        z.ZodUnion<[T["schema"], typeof NodeOutputValue.schema]>;
