import type { TypeSpec } from "../data/type-spec.ts";
import type { ConfigForType } from "./config-schema-for.ts";
import type { ArrayMeta } from "../data/types/array-def.tsx";
import type { RecordMeta } from "../data/types/record-def.tsx";
import { configExprByType, type ConfigExprType } from "./config-node.ts";
import { Throw } from "../util/throw.ts";
import type { ConfigEvalContext } from "./config-eval-context.ts";
import type { ConfigNode } from "./config-node.ts";
import type { TupleMeta } from "@/data/types/tuple-def.tsx";

/**
 * Evaluate a config value producing a value, a context.
 */
export function evaluateConfig({
  spec,
  configNode,
  context,
  path = [],
}: {
  spec: TypeSpec;
  configNode: ConfigNode;
  context: ConfigEvalContext;
  path?: string[];
}): unknown {
  const activeExprKey = configNode.activeExpr;

  if (activeExprKey != null) {
    const valueDef = configExprByType[activeExprKey];

    if (valueDef == null) {
      throw new Error(`Unknown config expr: ${activeExprKey}`);
    }

    return valueDef.evalFn({
      context,
      value: configNode[activeExprKey],
    });
  }

  const value = configNode.value;

  switch (spec.info.name) {
    case "array":
      return (value as ConfigNode[]).map((item, index) =>
        evaluateConfig({
          spec: (spec.info.meta as ArrayMeta).itemType,
          configNode: item,
          context,
          path: [...path, index.toString()],
        })
      );

    case "record":
      return Object.fromEntries(
        Object.entries(value as Record<string, ConfigNode>).map(
          ([key, value]) => [
            key,
            evaluateConfig({
              spec: (spec.info.meta as RecordMeta).shape[key],
              configNode: value,
              context,
              path: [...path, key],
            }),
          ]
        )
      );

    case "tuple":
      return (value as ConfigNode[]).map((item, index) =>
        evaluateConfig({
          spec: (spec.info.meta as TupleMeta).itemTypes[index],
          configNode: item,
          context,
          path: [...path, index.toString()],
        })
      );

    default:
      return value;
  }
}
