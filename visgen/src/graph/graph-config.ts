import { z } from "zod";
import { ObjectDef } from "../util/zod/object-def";
import { NodeConfig } from "./node-config";

export const GraphConfig = ObjectDef({
  nodes: z.record(z.string(), NodeConfig.schema),
});
export type GraphConfig = ReturnType<typeof GraphConfig>;
