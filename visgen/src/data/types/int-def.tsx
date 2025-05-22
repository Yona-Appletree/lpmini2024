import { z } from "zod";
import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";

export const IntDef = defineType("int32", (meta: TypeMeta<number>) =>
  TypeSpec(
    "int32",
    { ...meta, glType: "int32" },
    z.number().int().min(-2147483648).max(2147483647),
    ({ currentValue, onChange }) => {
      return (
        <input
          type="number"
          value={currentValue}
          onChange={(e) => onChange(Number(e.target.value))}
        />
      );
    },
  ),
);
