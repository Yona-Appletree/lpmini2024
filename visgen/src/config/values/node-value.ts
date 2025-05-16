import { ValueDef } from "../value-def.ts";
import { NodeId } from "../../graph/node-id.ts";

export const NodeOutputValue = ValueDef("node-output", {
  nodeId: NodeId.schema,
});
export type NodeOutputValue = ReturnType<typeof NodeOutputValue>;
