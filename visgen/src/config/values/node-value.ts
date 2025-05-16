import { ValueDef } from "../value-def.ts";
import { NodeId } from "../../node/node-id.ts";
import { TypeDefs } from "../../type/type-defs.ts";

export const NodeOutputValue = ValueDef("node-output", {
  nodeId: NodeId.schema,
  type: TypeDefs,
});
