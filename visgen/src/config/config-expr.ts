import { UnionDef } from "../util/zod/union-def.ts";
import { ModuleOutputExpr } from "./expressions/module-output-expr.ts";

// -----------------------------------------------------------------------------

const configExprDefs = [ModuleOutputExpr] as const;

// -----------------------------------------------------------------------------

export const configExprByType = Object.fromEntries(
  configExprDefs.map((def) => [def.type, def]),
) as {
  [I in keyof typeof configExprDefs as (typeof configExprDefs)[I] extends {
    type: string;
  }
    ? (typeof configExprDefs)[I]["type"]
    : never]: (typeof configExprDefs)[I];
};

export type ConfigExprType = keyof typeof configExprByType;

// -----------------------------------------------------------------------------

export const ConfigExpr = UnionDef(
  "$expr",
  configExprDefs.map(
    (def) => def.schema,
  ) as unknown as MapConfigExprDefsToSchemas<typeof configExprDefs>,
);

export type ConfigExpr =
  (typeof configExprByType)[keyof typeof configExprByType];

// -----------------------------------------------------------------------------

type MapConfigExprDefsToSchemas<T extends readonly unknown[]> =
  T extends readonly [infer First, ...infer Rest]
    ? readonly [
        First extends { schema: unknown } ? First["schema"] : never,
        ...MapConfigExprDefsToSchemas<Rest>,
      ]
    : readonly [];
