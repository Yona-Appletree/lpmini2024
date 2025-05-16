import { type BaseTypeMeta, GenericTypeDef, TypeDef } from "../type-def.ts";
import { z } from "zod";

export const EnumDef = GenericTypeDef(
  "enum",
  <TValues extends [string, ...string[]]>(
    meta: BaseTypeMeta & {
      values: TValues;
    },
  ) => TypeDef("enum", meta, z.enum(meta.values)),
);
