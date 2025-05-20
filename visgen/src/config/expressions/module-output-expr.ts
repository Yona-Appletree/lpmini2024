import { defineConfigExpr } from "../define-config-expr.ts";
import { ModuleId } from "@/program/module-id.ts";

export const ModuleOutputExpr = defineConfigExpr(
  "node-output",
  {
    nodeId: ModuleId.schema,
  },
  ({ context, value }) => {
    return context.nodeMap.get(value.nodeId)?.output;
  },
);
export type NodeOutputExpr = ReturnType<typeof ModuleOutputExpr>;
