import type { TypeSpec } from "../data/type-spec.ts";
import type { ConfigValue } from "./config-schema-for.ts";
import type { ArrayMeta } from "../data/types/array-def.tsx";
import type { RecordMeta } from "../data/types/record-def.tsx";
import type { ModuleOutputExpr } from "./expressions/module-output-expr.tsx";

export function renderConfig({
  spec,
  config,
  moduleMap,
}: {
  spec: TypeSpec;
  config: unknown;
  moduleMap: Map<string, { output: unknown }>;
}): unknown {
  switch (spec.info.name) {
    case "array":
      return (config as ConfigValue[]).map((item) =>
        renderConfig({
          spec: (spec.info.meta as ArrayMeta).itemType,
          config: item,
          moduleMap,
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
              moduleMap,
            }),
          ],
        ),
      );

    default:
      if (typeof config === "object" && config !== null && "$expr" in config) {
        switch (config.$expr) {
          case "node-output":
            return moduleMap.get((config as ModuleOutputExpr).moduleId)?.output;
        }
      } else {
        return config;
      }
  }
}
