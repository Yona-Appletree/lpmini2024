import { z } from "zod";
import { TypeDef } from "../type-def.ts";

export const Int32 = TypeDef(
  "int32",
  z.number().int().min(-2147483648).max(2147483647),
);
export type Int32 = ReturnType<typeof Int32>;
