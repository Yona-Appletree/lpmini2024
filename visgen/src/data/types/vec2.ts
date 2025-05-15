import { z } from "zod";
import { TypeDef } from "../type-def.ts";

export const Vec2 = TypeDef("vec2", z.tuple([z.number(), z.number()]));
export type Vec2 = ReturnType<typeof Vec2>;
