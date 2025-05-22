import type { TypeSpec } from "../data/type-spec.ts";
import type { TypeName } from "../data/type-def.ts";
import type { ArrayDef, ArrayMeta } from "../data/types/array-def.tsx";
import { RecordDef, type RecordMeta } from "../data/types/record-def.tsx";
import { z } from "zod";
import { mapValues } from "../util/map-values.ts";
import { ConfigExprKey, ConfigNodeExpr } from "./config-node.ts";
import type {
  TupleDef,
  TupleMeta,
  TupleSchemas,
} from "@/data/types/tuple-def.tsx";

export function configSchemaFor<TSpec extends TypeSpec<TypeName>>(
  spec: TSpec,
): ConfigSchemaFor<TSpec> {
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
  }) as ConfigSchemaFor<TSpec>;
}

export type ConfigSchemaFor<T extends TypeSpec<TypeName> = TypeSpec<TypeName>> =
  z.ZodObject<
    (typeof ConfigNodeExpr)["schema"]["shape"] & {
      activeExpr: z.ZodOptional<(typeof ConfigExprKey)["schema"]>;
      value: // Array
      z.ZodOptional<
        T["info"] extends ArrayDef
          ? T["info"]["meta"] extends ArrayMeta
            ? z.ZodArray<ConfigSchemaFor<T["info"]["meta"]["itemType"]>>
            : z.ZodLiteral<"bad1">
          : // Record
            T["info"] extends RecordDef
            ? T["info"]["meta"] extends RecordMeta
              ? z.ZodObject<{
                  [TKey in keyof T["info"]["meta"]["shape"]]: ConfigSchemaFor<
                    T["info"]["meta"]["shape"][TKey]
                  >;
                }>
              : z.ZodLiteral<"bad2">
            : // Tuple
              T["info"] extends TupleDef
              ? T["info"]["meta"] extends TupleMeta
                ? z.ZodTuple<TupleSchemas<T["info"]["meta"]["itemTypes"]>>
                : z.ZodLiteral<"bad3">
              : // everything else
                T["schema"]
      >;
    }
  >;
