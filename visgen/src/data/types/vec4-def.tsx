import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";
import { z } from "zod";

export const Vec4Def = defineType("vec4", (meta: Vec4Meta) =>
  TypeSpec(
    "vec4",
    meta,
    z.tuple([z.number(), z.number(), z.number(), z.number()]),
    ({ context, currentValue, onChange }) => {
      return (
        <div>
          <input
            type="number"
            value={currentValue[0]}
            onChange={(e) =>
              onChange([
                Number(e.target.value),
                currentValue[1],
                currentValue[2],
                currentValue[3],
              ])
            }
          />
          <input
            type="number"
            value={currentValue[1]}
            onChange={(e) =>
              onChange([
                currentValue[0],
                Number(e.target.value),
                currentValue[2],
                currentValue[3],
              ])
            }
          />
          <input
            type="number"
            value={currentValue[2]}
            onChange={(e) =>
              onChange([
                currentValue[0],
                currentValue[1],
                Number(e.target.value),
                currentValue[3],
              ])
            }
          />
          <input
            type="number"
            value={currentValue[3]}
            onChange={(e) =>
              onChange([
                currentValue[0],
                currentValue[1],
                currentValue[2],
                Number(e.target.value),
              ])
            }
          />
        </div>
      );
    }
  )
);

export type Vec4Def = ReturnType<typeof Vec4Def>;
export type Vec4 = z.output<Vec4Def["schema"]>;

export interface Vec4Meta extends TypeMeta<[number, number, number, number]> {
  quantity?: {
    type: "color";
    encoding: "normalized";
  };
}
