import { IdDef } from "../util/zod/id-def.ts";

export const NodeId = IdDef("node");
export type NodeId = ReturnType<typeof NodeId>;
