import { type BaseTypeMeta, GenericTypeDef } from "../type-spec-fn.ts";
import { z } from "zod";

export const EnumDef = GenericTypeDef(
  "enum",
  <TValues extends [string, ...string[]]>(
    meta: BaseTypeMeta & {
      values: TValues;
    },
  ) => TypeDef("enum", meta, z.enum(meta.values)),
);
