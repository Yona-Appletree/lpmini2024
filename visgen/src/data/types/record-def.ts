import {
  type BaseTypeMeta,
  GenericTypeDef,
  TypeDef,
  type TypeSpec,
} from "../type-def.ts";
import { mapValues } from "../../util/map-values.ts";
import { z } from "zod";

export const RecordDef = GenericTypeDef(
  "record",
  <TShape extends Record<string, TypeSpec>>(
    shape: TShape,
    meta: BaseTypeMeta = {},
  ) =>
    TypeDef(
      "record",
      {
        ...meta,
        shape: mapValues(shape, (it) => it.info),
      },
      z.object(
        mapValues(shape, (it) => it.schema) as {
          [TKey in keyof TShape]: TShape[TKey]["schema"];
        },
      ),
    ),
);
