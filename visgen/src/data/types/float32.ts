import { z } from "zod";
import { TypeDef } from "../type-def.ts";

export const Float32 = TypeDef("float32", z.number());
export type Float32 = ReturnType<typeof Float32>;
