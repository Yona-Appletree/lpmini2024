import { GenericTypeDef, TypeDef, type TypeMeta } from "../type-def.ts";
import { mapValues } from "../../util/map-values.ts";
import { z } from "zod";

export const RecordOf = GenericTypeDef(
  "Record",
  <TShape extends Record<string, TypeMeta>>(shape: TShape) =>
    TypeDef(
      ["Record", mapValues(shape, (it) => it.specifier)] as const,
      z.object(
        mapValues(shape, (it) => it.schema) as {
          [TKey in keyof TShape]: TShape[TKey]["schema"];
        },
      ),
    ),
);
