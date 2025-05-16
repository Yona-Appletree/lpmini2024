import type { z } from "zod";
import type {TypeSpec} from "../type/type-spec-fn.ts";
import type {TypeName} from "../type/type-defs.ts";
import type {ArrayDef, ArrayMeta} from "../type/types/array-def.ts";
import {RecordDef, type RecordMeta} from "../type/types/record-def.ts";




export type ConfigFor<T extends TypeSpec<TypeName>>
  =
  // array
  T["info"] extends ArrayDef
  ? T["info"]["meta"] extends ArrayMeta
    ? Array<ConfigFor<T["info"]["meta"]["itemType"]>>
    : never

    // record
  T["info"] extends RecordDef
  ? T["info"]["meta"] extends RecordMeta
    ? {
      [TKey in keyof T["info"]["meta"]["shape"]]: ConfigFor<T["info"]["meta"]["shape"][TKey]>
    }
    : never

  // everything else
  : z.infer<T["schema"]>;
