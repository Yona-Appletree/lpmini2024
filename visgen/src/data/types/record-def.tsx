import {
  defineType,
  type TypeMeta,
  TypeSpec,
  type TypeSpecOf,
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
      ({ context, currentValue, onChange }) => {
        return (
          <div>
            {Object.entries(shape).map(([prop, valueSpec]) => {
              const InputComponent =
                valueSpec.component ??
                Throw("Component not found for type: " + valueSpec.info.name);
              return (
                <div key={prop} className="flex gap-2">
                  <label>{prop}</label>
                  <InputComponent
                    context={context}
                    meta={valueSpec.info.meta}
                    currentValue={currentValue[prop]}
                    onChange={(value) =>
                      onChange({ ...currentValue, [prop]: value })
                    }
                  />
                </div>
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
