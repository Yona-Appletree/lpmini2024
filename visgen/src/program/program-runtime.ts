import { ProgramConfig } from "./program-config.ts";
import {
  ModuleConfig,
  type ModuleDef,
  moduleDefByType,
} from "./module-config.ts";
import type { NodeInstance } from "./module-def.ts";
import { evaluateConfig } from "../config/evaluate-config.ts";

export function ProgramRuntime(config: ProgramConfig) {
  const moduleMap = new Map<string, ModuleInstance>();
  const tickHandlers: (() => void)[] = [];

  let isRunning = true;

  // Initialize modules
  for (const [id, nodeConfig] of Object.entries(config.nodes)) {
    const nodeDef = moduleDefByType[nodeConfig.type];
    const instance = nodeDef();

    moduleMap.set(id, {
      id,
      nodeDef,
      nodeConfig,
      instance,
      input: nodeDef.metadata.input.info.meta.default,
      output: instance.update({
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        input: nodeDef.metadata.input.info.meta.default as any,
      }),
    });
  }

  const tick = () => {
    if (!isRunning) return;

    for (const [id, node] of moduleMap.entries()) {
      const nodeDef = moduleDefByType[config.nodes[id].type];
      const input = evaluateConfig({
        spec: nodeDef.metadata.input,
        config: config.nodes[id].input,
        context: { moduleMap },
      }) as unknown;
      const output = node.instance.update({ input });

      node.input = input;
      node.output = output;
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
  nodeConfig: ModuleConfig;
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
