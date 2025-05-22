import { z } from "zod";
import {
  defineType,
  type TypeMeta,
  TypeSpec,
  type TypeSpecOf,
} from "../type-spec.ts";
import { Input } from "@/components/ui/input.tsx";

export const FloatDef = defineType("float32", (meta: FloatMeta) =>
  TypeSpec(
    "float32",
    { glType: "float32", ...meta },
    z.number(),
    ({ meta, currentValue, onChange }) => {
      const ui = (meta as FloatMeta).ui ?? { type: "number" };

      switch (ui.type) {
        case "slider":
          return (
            <Input
              type="range"
              min={ui.min}
              max={ui.max}
              step={ui.step}
              value={currentValue}
              onChange={(e) => onChange(Number(e.target.value))}
            />
          );

        case "number":
          return (
            <Input
              type="number"
              className="w-[72px]"
              value={currentValue}
              onChange={(e) => onChange(Number(e.target.value))}
            />
          );
      }
    },
  ),
);
export type FloatDef = ReturnType<typeof FloatDef>;
export type FloatSpec = TypeSpecOf<typeof FloatDef>;

export interface FloatMeta extends TypeMeta<number> {
  unit?: string;
  ui?:
    | {
        type: "slider";
        min: number;
        max: number;
        step: number;
      }
    | {
        type: "number";
        min: number;
        max: number;
      };
}
