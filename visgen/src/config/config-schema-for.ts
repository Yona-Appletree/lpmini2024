import type { TypeSpec } from "../type/type-spec.ts";
import type { TypeName } from "../type/type-def.ts";
import type { ArrayDef, ArrayMeta } from "../type/types/array-def.ts";
import { RecordDef, type RecordMeta } from "../type/types/record-def.ts";
import { NodeOutputValue } from "./values/node-value.ts";
import { z } from "zod";
import { mapValues } from "../util/map-values.ts";

export function configSchemaFor<TSpec extends TypeSpec<TypeName>>(
  spec: TSpec,
): ConfigValue<TSpec> {
  switch (spec.info.name) {
    case "array":
      return z.array(
        configSchemaFor((spec.info.meta as ArrayMeta).itemType),
      ) as ConfigValue<TSpec>;
    case "record":
      return z.object(
        mapValues((spec.info.meta as RecordMeta).shape, (type) =>
          configSchemaFor(type),
        ),
      ) as ConfigValue<TSpec>;
    default:
      return z.union([
        spec.schema,
        NodeOutputValue.schema,
      ]) as ConfigValue<TSpec>;
  }
}

export type ConfigValue<T extends TypeSpec<TypeName> = TypeSpec<TypeName>> =
  // array
  T["info"] extends ArrayDef
    ? T["info"]["meta"] extends ArrayMeta
      ? z.ZodArray<ConfigValue<T["info"]["meta"]["itemType"]>>
      : never
    : // record
      T["info"] extends RecordDef
      ? T["info"]["meta"] extends RecordMeta
        ? z.ZodObject<{
            [TKey in keyof T["info"]["meta"]["shape"]]: ConfigValue<
              T["info"]["meta"]["shape"][TKey]
            >;
          }>
        : never
      : // everything else
        z.ZodUnion<[T["schema"], typeof NodeOutputValue.schema]>;
