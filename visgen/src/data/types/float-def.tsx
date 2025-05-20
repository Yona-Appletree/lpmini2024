import { z } from "zod";
import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";

export const FloatDef = defineType("float32", (meta: FloatMeta) =>
  TypeSpec("float32", meta, z.number(), ({ meta, currentValue, onChange }) => {
    const ui = meta.ui ?? { type: "number" };

    switch (ui.type) {
      case "slider":
        return (
          <input
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
          <input
            type="number"
            value={currentValue}
            onChange={(e) => onChange(Number(e.target.value))}
          />
        );
    }
  }),
);

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
