import { z } from "zod";
import { TypeDef } from "../type-def.ts";

export const Vec4 = TypeDef(
  "vec4",
  z.tuple([z.number(), z.number(), z.number(), z.number()]),
);
export type Vec4 = ReturnType<typeof Vec4>;
