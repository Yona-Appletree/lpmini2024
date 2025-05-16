import { type TypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";
import { z } from "zod";

export const EnumDef = GenericTypeDef(
  "enum",
  <TValues extends [string, ...string[]]>(
    meta: TypeMeta<TValues[number]> & {
      options: TValues;
    },
  ) => TypeSpec("enum", meta, z.enum(meta.options)),
);
