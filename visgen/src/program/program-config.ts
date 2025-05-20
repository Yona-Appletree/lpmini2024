import { z } from "zod";
import { ObjectDef } from "../util/zod/object-def";
import { ModuleConfig } from "./module-config.ts";

export const ProgramConfig = ObjectDef({
  nodes: z.record(z.string(), ModuleConfig.schema),
});
export type ProgramConfig = ReturnType<typeof ProgramConfig>;
