import { z } from "zod";
import { TypeDef } from "../type-def.ts";

export const Vec3 = TypeDef(
  "vec3",
  z.tuple([z.number(), z.number(), z.number()]),
);
export type Vec3 = ReturnType<typeof Vec3>;
