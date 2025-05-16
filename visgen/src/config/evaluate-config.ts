import type { TypeSpec } from "../type/type-spec.ts";
import type { ConfigValue } from "./config-schema-for.ts";
import type { ArrayMeta } from "../type/types/array-def.ts";
import type { RecordMeta } from "../type/types/record-def.ts";
import { configExprByType, type ConfigExprType } from "./config-expr.ts";
import { Throw } from "../util/throw.ts";

/**
 * Context in which configuration values are evaluated.
 */
export interface ConfigEvalContext {
  nodeMap: Map<string, { output: unknown }>;
}

/**
 * Evaluate a config value producing a value, a context.
 */
export function evaluateConfig({
  spec,
  config,
  context,
  path = [],
}: {
  spec: TypeSpec;
  config: unknown;
  context: ConfigEvalContext;
  path?: string[];
}): unknown {
  switch (spec.info.name) {
    case "array":
      return (config as ConfigValue[]).map((item, index) =>
        evaluateConfig({
          spec: (spec.info.meta as ArrayMeta).itemType,
          config: item,
          context,
          path: [...path, index.toString()],
        }),
      );

    case "record":
      return Object.fromEntries(
        Object.entries(config as Record<string, ConfigValue>).map(
          ([key, value]) => [
            key,
            evaluateConfig({
              spec: (spec.info.meta as RecordMeta).shape[key],
              config: value,
              context,
              path: [...path, key],
            }),
          ],
        ),
      );

    default:
      if (typeof config === "object" && config !== null && "$expr" in config) {
        const valueDef =
          configExprByType[config.$expr as ConfigExprType] ??
          Throw(
            `Unsupported config value: path=${path}, $expr=${config.$expr}`,
          );

        return valueDef.evalFn({
          context,
          value: valueDef.schema.parse(config),
        });
      } else {
        return config;
      }
  }
}
