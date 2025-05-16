import type { TypeSpec } from "../type/type-spec.ts";
import type { ConfigValue } from "./config-schema-for.ts";
import type { ArrayMeta } from "../type/types/array-def.ts";
import type { RecordMeta } from "../type/types/record-def.ts";
import type { NodeOutputExpr } from "./expressions/node-output-expr.ts";

export function renderConfig({
  spec,
  config,
  nodeMap,
}: {
  spec: TypeSpec;
  config: unknown;
  nodeMap: Map<string, { output: unknown }>;
}): unknown {
  switch (spec.info.name) {
    case "array":
      return (config as ConfigValue[]).map((item) =>
        renderConfig({
          spec: (spec.info.meta as ArrayMeta).itemType,
          config: item,
          nodeMap,
        }),
      );

    case "record":
      return Object.fromEntries(
        Object.entries(config as Record<string, ConfigValue>).map(
          ([key, value]) => [
            key,
            renderConfig({
              spec: (spec.info.meta as RecordMeta).shape[key],
              config: value,
              nodeMap,
            }),
          ],
        ),
      );

    default:
      if (typeof config === "object" && config !== null && "$expr" in config) {
        switch (config.$expr) {
          case "node-output":
            return nodeMap.get((config as NodeOutputExpr).nodeId)?.output;
        }
      } else {
        return config;
      }
  }
}
