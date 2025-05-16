import { type BaseTypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";
import { z } from "zod";

export const EnumDef = GenericTypeDef(
  "enum",
  <TValues extends [string, ...string[]]>(
    meta: BaseTypeMeta & {
      values: TValues;
    },
  ) => TypeSpec("enum", meta, z.enum(meta.values)),
);
