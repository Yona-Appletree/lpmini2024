import { z } from "zod";
import { ObjectDef } from "@/frontend/util/zod/object-def.ts";
import { _index } from "./modules/_index.ts";

export const ProgramConfig = ObjectDef({
  nodes: z.record(z.string(), _index.schema),
});
export type ProgramConfig = ReturnType<typeof ProgramConfig>;
