import { ProgramConfig } from "./program-config.ts";
import { _index, type ModuleDef, moduleDefByType } from "./modules/_index.ts";
import type { NodeInstance } from "./module-def.ts";
import { evaluateConfig } from "../config/evaluate-config.ts";
import { Gl2d } from "@/core/gl2d/gl2d.ts";

export function ProgramRuntime(config: ProgramConfig) {
  const moduleMap = new Map<string, ModuleInstance>();
  const tickHandlers: (() => void)[] = [];

  let isRunning = true;
  const gl2d = Gl2d();

  // Initialize modules
  for (const [id, nodeConfig] of Object.entries(config.nodes)) {
    const nodeDef = moduleDefByType[nodeConfig.type];
    const instance = nodeDef({ gl2d });

    moduleMap.set(id, {
      id,
      nodeDef,
      nodeConfig,
      instance,
      input: nodeDef.metadata.input.info.meta.default,
      output: nodeDef.metadata.output.info.meta.default,
    });
  }

  const tick = () => {
    if (!isRunning) return;

    for (const [id, node] of moduleMap.entries()) {
      const nodeDef = moduleDefByType[config.nodes[id].type];
      const input = evaluateConfig({
        spec: nodeDef.metadata.input,
        configNode: config.nodes[id].input,
        context: { moduleMap },
      }) as unknown;

      try {
        const output = node.instance.update({ input });

        node.input = input;
        node.output = output;
      } catch (e) {
        debugger;
        console.error("Failed to render node: ", id, e);
      }
    }

    for (const handler of tickHandlers) {
      handler();
    }

    requestAnimationFrame(tick);
  };

  const start = () => {
    if (isRunning) return;

    isRunning = true;
    requestAnimationFrame(tick);
  };

  const stop = () => {
    isRunning = false;
  };

  const addTickHandler = (handler: () => void) => {
    tickHandlers.push(handler);
    return () => {
      tickHandlers.splice(tickHandlers.indexOf(handler), 1);
    };
  };

  return {
    moduleMap,
    tick,
    addTickHandler,
    start,
    stop,
    programConfig: config,
  };
}

export type ProgramRuntime = ReturnType<typeof ProgramRuntime>;

export interface ModuleInstance {
  id: string;
  nodeConfig: _index;
  nodeDef: ModuleDef;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  instance: NodeInstance<any>;
  output: unknown;
  input: unknown;
}

export interface RuntimeContext {
  moduleMap: Map<string, ModuleInstance>;
  addTickHandler: (handler: () => void) => () => void;
  programConfig: ProgramConfig;
}
