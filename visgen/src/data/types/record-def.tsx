import {
  defineType,
  type TypeMeta,
  TypeSpec,
  type TypeSpecOf,
  type TypeValue,
} from "../type-spec.ts";
import { mapValues } from "../../util/map-values.ts";
import { z } from "zod";
import { Throw } from "@/util/throw.ts";

export const RecordDef = defineType(
  "record",
  <TShape extends Record<string, TypeSpec>>(
    shape: TShape,
    meta: Omit<TypeMeta<unknown>, "default"> = {},
  ) => {
    type TValue = {
      [TKey in keyof TShape]: TypeValue<TShape[TKey]>;
    };

    return TypeSpec<"record", TValue, RecordMeta<TShape>>(
      "record",
      {
        ...meta,
        default: mapValues(shape, (it) => it.info.meta.default) as TValue,
        shape,
      },
      z.object(mapValues(shape, (it) => it.schema)) as z.Schema<TValue>,
      ({ currentValue, onChange }) => {
        return (
          <div className="grid grid-cols-[auto_1fr] gap-2 p-1 items-baseline justify-items-start">
            {Object.entries(shape).map(([prop, valueSpec]) => {
              const InputComponent =
                valueSpec.component ??
                Throw("Component not found for type: " + valueSpec.info.name);
              return (
                <>
                  <label key={prop + "-label"} className="text-right">
                    {prop}
                  </label>
                  <InputComponent
                    key={prop + "-value"}
                    meta={valueSpec.info.meta}
                    currentValue={currentValue[prop]}
                    onChange={(value) =>
                      onChange({ ...currentValue, [prop]: value })
                    }
                  />
                </>
              );
            })}
          </div>
        );
      },
    );
  },
);

export type RecordDef = ReturnType<typeof RecordDef>;
export type RecordSpec = TypeSpecOf<typeof RecordDef>;

export interface RecordMeta<
  TShape extends Record<string, TypeSpec> = Record<string, TypeSpec>,
> extends TypeMeta<{
    [TKey in keyof TShape]: z.output<TShape[TKey]["schema"]>;
  }> {
  shape: TShape;
}
