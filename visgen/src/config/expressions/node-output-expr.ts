import { defineConfigExpr } from "../define-config-expr.ts";
import { NodeId } from "../../graph/node-id.ts";

export const NodeOutputExpr = defineConfigExpr(
  "node-output",
  {
    nodeId: NodeId.schema,
  },
  ({ context, value }) => {
    return context.nodeMap.get(value.nodeId)?.output;
  },
);
export type NodeOutputExpr = ReturnType<typeof NodeOutputExpr>;
