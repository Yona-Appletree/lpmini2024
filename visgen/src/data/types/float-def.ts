import { z } from "zod";
import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";

export const FloatDef = defineType("float32", (meta: FloatMeta) =>
  TypeSpec("float32", meta, z.number()),
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
