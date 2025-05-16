import type { TypeSpec } from "../type/type-spec.ts";
import type { TypeName } from "../type/type-def.ts";
import type { ArrayDef, ArrayMeta } from "../type/types/array-def.ts";
import { RecordDef, type RecordMeta } from "../type/types/record-def.ts";
import { NodeOutputValue } from "./values/node-value.ts";

export type ConfigSchemaFor<T extends TypeSpec<TypeName>> =
  // array
  T["info"] extends ArrayDef
    ? T["info"]["meta"] extends ArrayMeta
      ? Array<ConfigSchemaFor<T["info"]["meta"]["itemType"]>>
      : never
    : // record
      T["info"] extends RecordDef
      ? T["info"]["meta"] extends RecordMeta
        ? {
            [TKey in keyof T["info"]["meta"]["shape"]]: ConfigSchemaFor<
              T["info"]["meta"]["shape"][TKey]
            >;
          }
        : never
      : // everything else
        T["schema"] | typeof NodeOutputValue.schema;
