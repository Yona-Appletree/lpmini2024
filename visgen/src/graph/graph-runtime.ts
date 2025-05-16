import { GraphConfig } from "./graph-config";
import { nodeDefByType, type NodeDef } from "./node-config";
import type { NodeInstance } from "./node-def";
import { renderConfig } from "../config/render-config.ts";

export function GraphRuntime(config: GraphConfig) {
  const nodeMap = new Map<
    string,
    {
      nodeDef: NodeDef;
      instance: NodeInstance<any>;
      output: unknown;
      input: unknown;
    }
  >();

  // Initialize nodes
  for (const [id, node] of Object.entries(config.nodes)) {
    const nodeDef = nodeDefByType[node.type];
    const instance = nodeDef();
    const input = nodeDef.metadata.input.info.meta.default;

    nodeMap.set(id, {
      nodeDef,
      instance,
      input,
      output: instance.update({
        input: nodeDef.metadata.input.info.meta.default,
      }),
    });
  }

  return {
    nodeMap,
    tick: () => {
      for (const [id, node] of nodeMap.entries()) {
        const nodeDef = nodeDefByType[config.nodes[id].type];
        const input = renderConfig({
          spec: nodeDef.metadata.input,
          config: config.nodes[id].input,
          nodeMap,
        }) as unknown;
        const output = node.instance.update({ input });

        node.input = input;
        node.output = output;
      }
    },
  };
}

export type GraphRuntime = ReturnType<typeof GraphRuntime>;
