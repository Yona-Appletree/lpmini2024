import {
  type BaseTypeMeta,
  GenericTypeDef,
  type TypeSpec,
} from "../type-spec-fn.ts";
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

export type RecordDef = ReturnType<typeof RecordDef>;

export type RecordMeta<
  TShape extends Record<string, TypeSpec> = Record<string, TypeSpec>,
> = BaseTypeMeta & {
  shape: TShape;
};
