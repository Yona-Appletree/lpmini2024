import type { TypeSpec } from "@/core/data/type-spec.ts";
import type { ConfigForType } from "./config-schema-for.ts";
import type { ArrayMeta } from "@/core/data/types/array-def.tsx";
import type { RecordMeta } from "@/core/data/types/record-def.tsx";
import type { ModuleOutputExpr } from "./expressions/$moduleOutput-expr.tsx";

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
      return (config as ConfigForType[]).map((item) =>
        renderConfig({
          spec: (spec.info.meta as ArrayMeta).itemType,
          config: item,
          moduleMap,
        }),
      );

    case "record":
      return Object.fromEntries(
        Object.entries(config as Record<string, ConfigForType>).map(
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
          case "$moduleOutput":
            return moduleMap.get((config as ModuleOutputExpr).moduleId)?.output;
        }
      } else {
        return config;
      }
  }
}
