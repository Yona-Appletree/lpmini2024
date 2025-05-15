import { ZodDef } from "../zod/zod-def.ts";
import { z } from "zod";

export const Int32 = ZodDef(z.number().int().min(-2147483648).max(2147483647));
export type Int32 = ReturnType<typeof Int32>;
