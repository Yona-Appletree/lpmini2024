import { type TypeMeta, GenericTypeDef, TypeSpec } from "../type-spec.ts";
import { mapValues } from "../../util/map-values.ts";
import { z } from "zod";

export const RecordDef = GenericTypeDef(
  "record",
  <TShape extends Record<string, TypeSpec>>(
    shape: TShape,
    meta: Omit<TypeMeta<unknown>, "default"> = {},
  ) => {
    const schema = z.object(
      mapValues(shape, (it) => it.schema) as {
        [TKey in keyof TShape]: TShape[TKey]["schema"];
      },
    );

    return TypeSpec(
      "record",
      {
        ...meta,
        default: mapValues(shape, (it) => it.info.meta.default) as z.output<
          typeof schema
        >,
        shape,
      },
      schema,
    );
  },
);

export type RecordDef = ReturnType<typeof RecordDef>;

export interface RecordMeta<
  TShape extends Record<string, TypeSpec> = Record<string, TypeSpec>,
> extends TypeMeta<{
    [TKey in keyof TShape]: z.output<TShape[TKey]["schema"]>;
  }> {
  shape: TShape;
}
