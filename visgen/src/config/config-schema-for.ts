import type { TypeSpec } from "../data/type-spec.ts";
import type { TypeName } from "../data/type-def.ts";
import type { ArrayDef, ArrayMeta } from "../data/types/array-def.tsx";
import { RecordDef, type RecordMeta } from "../data/types/record-def.tsx";
import { z } from "zod";
import { mapValues } from "../util/map-values.ts";
import { ConfigExpr } from "./config-expr.ts";
import type { TupleMeta } from "@/data/types/tuple-def.tsx";

export function configSchemaFor<TSpec extends TypeSpec<TypeName>>(
  spec: TSpec,
): ConfigValue<TSpec> {
  switch (spec.info.name) {
    case "array":
      return z.array(
        configSchemaFor((spec.info.meta as ArrayMeta).itemType),
      ) as ConfigValue<TSpec>;

    case "tuple":
      return z.tuple(
        (spec.info.meta as TupleMeta).itemTypes.map((it) =>
          configSchemaFor(it),
        ),
      ) as ConfigValue<TSpec>;

    case "record":
      return z.object(
        mapValues((spec.info.meta as RecordMeta).shape, (type) =>
          configSchemaFor(type),
        ),
      ) as ConfigValue<TSpec>;

    default:
      return z.union([spec.schema, ConfigExpr.schema]) as ConfigValue<TSpec>;
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
        z.ZodUnion<[T["schema"], typeof ConfigExpr.schema]>;
