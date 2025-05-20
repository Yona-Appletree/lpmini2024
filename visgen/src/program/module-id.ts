import { IdDef } from "../util/zod/id-def.ts";

export const ModuleId = IdDef("node");
export type ModuleId = ReturnType<typeof ModuleId>;
