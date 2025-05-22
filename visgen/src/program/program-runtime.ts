import { ProgramConfig } from "./program-config.ts";
import { type ModuleDef, moduleDefByType } from "./module-config.ts";
import type { NodeInstance } from "./module-def.ts";
import { evaluateConfig } from "../config/evaluate-config.ts";

export function ProgramRuntime(config: ProgramConfig) {
  const nodeMap = new Map<string, RuntimeNode>();

  // Initialize modules
  for (const [id, node] of Object.entries(config.nodes)) {
    const nodeDef = moduleDefByType[node.type];
    const instance = nodeDef();

    nodeMap.set(id, {
      id,
      nodeDef,
      instance,
      input: nodeDef.metadata.input.info.meta.default,
      output: instance.update({
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        input: nodeDef.metadata.input.info.meta.default as any,
      }),
    });
  }

  return {
    nodeMap,
    tick: () => {
      for (const [id, node] of nodeMap.entries()) {
        const nodeDef = moduleDefByType[config.nodes[id].type];
        const input = evaluateConfig({
          spec: nodeDef.metadata.input,
          config: config.nodes[id].input,
          context: { nodeMap },
        }) as unknown;
        const output = node.instance.update({ input });

        node.input = input;
        node.output = output;
      }
    },
  };
}

export type ProgramRuntime = ReturnType<typeof ProgramRuntime>;

export interface RuntimeNode {
  id: string;
  nodeDef: ModuleDef;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  instance: NodeInstance<any>;
  output: unknown;
  input: unknown;
}

export interface RuntimeContext {
  nodeMap: Map<string, RuntimeNode>;
}
